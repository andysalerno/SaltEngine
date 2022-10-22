#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, rc::Rc, sync::Mutex};

    use engine::{
        deck::Deck,
        event::{Event, EventHandler},
        Card, CardDefinition, ClientChannel, Dispatcher, FromClient, FromServer, GamePos,
        GameState, MessageChannel, PlayerId,
    };
    use events::{
        CardDrawnClientEvent, CreatureAttacksTargetEvent, CreatureAttacksTargetEventHandler,
        CreaturePlacedOnBoardEvent, CreaturePlacedOnBoardEventHandler, CreatureTakesDamageEvent,
        CreatureTakesDamageEventHandler, DrawCardEvent, DrawCardEventHandler,
        PlayerSummonsCreatureEvent, PlayerSummonsCreatureEventHandler, StartGameEvent,
        StartGameEventHandler,
    };
    use log::info;

    #[test]
    fn on_game_start_expects_cards_drawn() {
        init_logger();

        let player_a_id = PlayerId::new();
        let player_b_id = PlayerId::new();

        let dispatcher = {
            let handlers: Vec<Box<dyn EventHandler>> = vec![
                Box::new(DrawCardEventHandler::new()),
                Box::new(StartGameEventHandler::new()),
            ];

            let player_a_channel = {
                let client = DummyClient::new();

                ClientChannel::new(player_a_id, Box::new(client))
            };

            let player_b_channel = {
                let client = DummyClient::new();

                ClientChannel::new(player_b_id, Box::new(client))
            };

            Dispatcher::new(handlers, player_a_channel, player_b_channel)
        };

        let mut game_state = {
            let mut builder = GameState::builder(player_a_id, player_b_id);
            builder
                .with_player_a_deck(make_deck(10))
                .with_player_b_deck(make_deck(10));
            builder.build()
        };

        // Pre-assert condition
        let hand_len = game_state.hand(player_a_id).len();
        assert!(hand_len == 0);

        dispatcher.dispatch(StartGameEvent::new(), &mut game_state);

        let hand_len = game_state.hand(player_a_id).len();

        assert!(hand_len == 6);
    }

    #[test]
    fn on_player_a_draws_card_expects_card_hidden_from_player_b() {
        init_logger();

        let handlers: Vec<Box<dyn EventHandler>> = vec![Box::new(DrawCardEventHandler::new())];

        let player_a_id = PlayerId::new();
        let player_b_id = PlayerId::new();

        let (dispatcher, a_observer, b_observer) =
            make_dispatcher(player_a_id, player_b_id, handlers);

        let mut game_state = {
            let mut builder = GameState::builder(player_a_id, player_b_id);
            builder
                .with_player_a_deck(make_deck(10))
                .with_player_b_deck(make_deck(10));
            builder.build()
        };

        // 1. player_a draws a card
        dispatcher.dispatch(DrawCardEvent::new(player_a_id), &mut game_state);

        // 2. Acquire message received by a
        let a_received = a_observer.pop_received().unwrap();

        // 3. Acquire message received by b
        let b_received = b_observer.pop_received().unwrap();

        let a_event: CardDrawnClientEvent = match a_received {
            FromServer::Event(e) => e,
            _ => panic!("expected an event"),
        }
        .unpack();

        let b_event: CardDrawnClientEvent = match b_received {
            FromServer::Event(e) => e,
            _ => panic!("expected an event"),
        }
        .unpack();

        assert!(
            a_event.card_drawn().is_visible(),
            "PlayerA drew the card, so it should be visible to them."
        );
        assert!(
            b_event.card_drawn().is_hidden(),
            "PlayerB should not see what card was drawn by PlayerA."
        );
    }

    #[test]
    fn on_creature_attack_expects_take_damage() {
        init_logger();

        let handlers: Vec<Box<dyn EventHandler>> = vec![
            Box::new(CreatureTakesDamageEventHandler::new()),
            Box::new(CreatureAttacksTargetEventHandler::new()),
        ];

        let player_a_id = PlayerId::new();
        let player_b_id = PlayerId::new();

        let (dispatcher, _, _) = make_dispatcher(player_a_id, player_b_id, handlers);

        let mut game_state = {
            let mut builder = GameState::builder(player_a_id, player_b_id);
            builder
                .with_player_a_deck(make_deck(10))
                .with_player_b_deck(make_deck(10));
            builder.build()
        };

        let attacker_card = Card::new(Box::new(
            CardDefinition::builder()
                .title("test_card")
                .health(5)
                .attack(1)
                .build(),
        ));
        let attacker_card_id = attacker_card.id();
        game_state.set_card_at_pos(GamePos::SlotIndex(0), attacker_card);

        let target_card = Card::new(Box::new(
            CardDefinition::builder()
                .title("test_card")
                .health(5)
                .build(),
        ));
        let target_card_id = target_card.id();
        game_state.set_card_at_pos(GamePos::SlotIndex(1), target_card);

        let attack_event =
            CreatureAttacksTargetEvent::new(player_a_id, attacker_card_id, target_card_id);

        let original_health = game_state.card(target_card_id).unwrap().current_health();
        dispatcher.dispatch(attack_event, &mut game_state);
        let final_health = game_state.card(target_card_id).unwrap().current_health();

        assert_eq!(
            final_health,
            original_health - 1,
            "Expected the card to take damage equal to the attacker's attack value."
        );
    }

    #[test]
    fn on_creature_attack_expects_events_dispatched_in_order() {
        init_logger();

        let handlers: Vec<Box<dyn EventHandler>> = vec![
            Box::new(CreatureTakesDamageEventHandler::new()),
            Box::new(CreatureAttacksTargetEventHandler::new()),
        ];

        let player_a_id = PlayerId::new();
        let player_b_id = PlayerId::new();

        let (dispatcher, player_a_observer, player_b_observer) =
            make_dispatcher(player_a_id, player_b_id, handlers);

        let mut game_state = GameState::builder(player_a_id, player_b_id).build();

        // Add attacker to board
        let attacker_card_id = {
            let attacker_card = Card::new(Box::new(
                CardDefinition::builder()
                    .title("test_card")
                    .health(5)
                    .attack(1)
                    .build(),
            ));
            let id = attacker_card.id();
            game_state.set_card_at_pos(GamePos::SlotIndex(0), attacker_card);

            id
        };

        // Add target to board
        let target_card_id = {
            let target_card = Card::new(Box::new(
                CardDefinition::builder()
                    .title("test_card")
                    .health(5)
                    .build(),
            ));
            let id = target_card.id();
            game_state.set_card_at_pos(GamePos::SlotIndex(1), target_card);

            id
        };

        // Attacker attacks target
        {
            let attack_event =
                CreatureAttacksTargetEvent::new(player_a_id, attacker_card_id, target_card_id);

            dispatcher.dispatch(attack_event, &mut game_state);
        }

        // Validate: players see attack event first
        {
            let event_1 = match player_a_observer.pop_received() {
                Some(FromServer::Event(e)) => e,
                _ => panic!("Expected event from server"),
            };

            assert_eq!(
                *event_1.event_type(),
                CreatureAttacksTargetEvent::event_type()
            );

            let event_1 = match player_b_observer.pop_received() {
                Some(FromServer::Event(e)) => e,
                _ => panic!("Expected event from server"),
            };

            assert_eq!(
                *event_1.event_type(),
                CreatureAttacksTargetEvent::event_type()
            );
        }

        // Validate: players see damage event next
        {
            let event_2 = match player_a_observer.pop_received() {
                Some(FromServer::Event(e)) => e,
                _ => panic!("Expected event from server"),
            };

            assert_eq!(
                *event_2.event_type(),
                CreatureTakesDamageEvent::event_type()
            );

            let event_2 = match player_b_observer.pop_received() {
                Some(FromServer::Event(e)) => e,
                _ => panic!("Expected event from server"),
            };

            assert_eq!(
                *event_2.event_type(),
                CreatureTakesDamageEvent::event_type()
            );
        }
    }

    #[test]
    fn creature_placed_event_expects_creature_placed() {
        init_logger();

        let handlers: Vec<Box<dyn EventHandler>> =
            vec![Box::new(CreaturePlacedOnBoardEventHandler::new())];

        let player_a_id = PlayerId::new();
        let player_b_id = PlayerId::new();

        let (dispatcher, client_a_observer, client_b_observer) =
            make_dispatcher(player_a_id, player_b_id, handlers);

        let mut game_state = GameState::builder(player_a_id, player_b_id).build();

        let card_to_place = Card::new(Box::new(
            CardDefinition::builder()
                .title("test_card")
                .health(5)
                .attack(1)
                .build(),
        ));

        let card_id = card_to_place.id();

        let pos = GamePos::SlotIndex(3);
        let event = CreaturePlacedOnBoardEvent::new(player_a_id, card_to_place, pos);
        dispatcher.dispatch(event, &mut game_state);

        let card_at_pos = game_state.card_at_pos(pos);

        assert!(card_at_pos.is_some());
        assert_eq!(card_id, card_at_pos.unwrap().id());

        let a_received = client_a_observer.pop_received();
        let b_received = client_b_observer.pop_received();

        match a_received {
            Some(FromServer::Event(e)) => {
                let event: CreaturePlacedOnBoardEvent = e.unpack();

                assert_eq!(event.card().id(), card_id);
            }
            _ => panic!("Expected a received message from server."),
        }

        match b_received {
            Some(FromServer::Event(e)) => {
                let event: CreaturePlacedOnBoardEvent = e.unpack();

                assert_eq!(event.card().id(), card_id);
            }
            _ => panic!("Expected a received message from server."),
        }
    }

    #[test]
    fn on_set_card_expects_get_by_id() {
        init_logger();

        let player_a_id = PlayerId::new();
        let player_b_id = PlayerId::new();

        let mut game_state = GameState::builder(player_a_id, player_b_id).build();

        let attacker_card = Card::new(Box::new(
            CardDefinition::builder()
                .title("test_card_1")
                .health(5)
                .attack(1)
                .build(),
        ));

        let card_id = attacker_card.id();

        game_state.set_card_at_pos(GamePos::SlotIndex(0), attacker_card);

        let found_by_id = game_state.card(card_id);

        assert_eq!("test_card_1", found_by_id.unwrap().definition().title());
    }

    #[test]
    fn on_set_card_expects_get_by_pos() {
        init_logger();

        let player_a_id = PlayerId::new();
        let player_b_id = PlayerId::new();

        let mut game_state = GameState::builder(player_a_id, player_b_id).build();

        let attacker_card = Card::new(Box::new(
            CardDefinition::builder()
                .title("test_card_1")
                .health(5)
                .attack(1)
                .build(),
        ));

        game_state.set_card_at_pos(GamePos::SlotIndex(0), attacker_card);

        let found_by_pos = game_state.card_at_pos(GamePos::SlotIndex(0));

        assert_eq!("test_card_1", found_by_pos.unwrap().definition().title());
    }

    #[test]
    fn on_set_card_expects_get_by_pos_empty_if_different_pos() {
        init_logger();

        let player_a_id = PlayerId::new();
        let player_b_id = PlayerId::new();

        let mut game_state = GameState::builder(player_a_id, player_b_id).build();

        let attacker_card = Card::new(Box::new(
            CardDefinition::builder()
                .title("test_card_1")
                .health(5)
                .attack(1)
                .build(),
        ));

        game_state.set_card_at_pos(GamePos::SlotIndex(0), attacker_card);

        let found_by_different_pos = game_state.card_at_pos(GamePos::SlotIndex(1));

        assert!(found_by_different_pos.is_none());
    }

    #[test]
    fn on_summon_from_hand_expects_card_removed_from_hand() {
        init_logger();

        let handlers: Vec<Box<dyn EventHandler>> = vec![
            Box::new(CreaturePlacedOnBoardEventHandler::new()),
            Box::new(PlayerSummonsCreatureEventHandler::new()),
            Box::new(StartGameEventHandler::new()),
            Box::new(DrawCardEventHandler::new()),
        ];

        let player_a_id = PlayerId::new();
        let player_b_id = PlayerId::new();

        let (dispatcher, _, _) = make_dispatcher(player_a_id, player_b_id, handlers);

        let mut game_state = {
            let mut builder = GameState::builder(player_a_id, player_b_id);
            builder
                .with_player_a_deck(make_deck(10))
                .with_player_b_deck(make_deck(10));
            builder.build()
        };

        let card_id = {
            let top_card = game_state.deck_mut(player_a_id).take_from_top().unwrap();
            let id = top_card.id();
            game_state.deck_mut(player_a_id).add_card_to_top(top_card);

            id
        };

        // start game (triggers card draw)
        dispatcher.dispatch(StartGameEvent::new(), &mut game_state);

        let pos = GamePos::SlotIndex(4);
        let summon_from_hand_event = PlayerSummonsCreatureEvent::new(player_a_id, card_id, pos);

        dispatcher.dispatch(summon_from_hand_event, &mut game_state);

        let card_at_pos = game_state.card_at_pos(pos);
        assert!(
            card_at_pos.is_some(),
            "Expected the card to have been placed on the target pos."
        );

        let maybe_card_in_hand = game_state.hand(player_a_id).card(card_id);

        assert!(
            maybe_card_in_hand.is_none(),
            "Expected card to not exist in hand anymore."
        );
    }

    fn make_dispatcher(
        player_a_id: PlayerId,
        player_b_id: PlayerId,
        handlers: impl IntoIterator<Item = Box<dyn EventHandler>>,
    ) -> (Dispatcher, ClientObserver, ClientObserver) {
        let handlers = handlers.into_iter().collect();

        let a_observer;
        let b_observer;

        let player_a_channel = {
            let client = DummyClient::new();
            a_observer = client.observer();

            ClientChannel::new(player_a_id, Box::new(client))
        };

        let player_b_channel = {
            let client = DummyClient::new();
            b_observer = client.observer();

            ClientChannel::new(player_b_id, Box::new(client))
        };

        (
            Dispatcher::new(handlers, player_a_channel, player_b_channel),
            a_observer,
            b_observer,
        )
    }

    fn make_deck(card_count: usize) -> Deck {
        let mut deck = Deck::new_empty();

        let mut builder = CardDefinition::builder();
        builder.title("test_card");

        for _ in 0..card_count {
            deck.add_card_to_bottom(Card::new(builder.build()));
        }

        deck
    }

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    struct DummyClient {
        observer: ClientObserver,
    }

    #[derive(Clone)]
    struct ClientObserver {
        received_messages: Rc<Mutex<VecDeque<FromServer>>>,
        messages_to_send: Rc<Mutex<VecDeque<FromClient>>>,
    }

    impl ClientObserver {
        fn new() -> Self {
            Self {
                received_messages: Rc::new(Mutex::new(VecDeque::new())),
                messages_to_send: Rc::new(Mutex::new(VecDeque::new())),
            }
        }

        fn enqueue_received(&self, message: FromServer) {
            self.received_messages.lock().unwrap().push_back(message);
        }

        fn pop_received(&self) -> Option<FromServer> {
            self.received_messages.lock().unwrap().pop_front()
        }

        fn enqueue_sent(&self, message: FromClient) {
            self.messages_to_send.lock().unwrap().push_back(message);
        }

        fn pop_sent(&self) -> Option<FromClient> {
            self.messages_to_send.lock().unwrap().pop_front()
        }
    }

    impl DummyClient {
        fn new() -> Self {
            Self {
                observer: ClientObserver::new(),
            }
        }

        fn observer(&self) -> ClientObserver {
            self.observer.clone()
        }
    }

    impl MessageChannel for DummyClient {
        type Send = FromServer;
        type Receive = FromClient;

        fn send(&self, message: FromServer) {
            info!("Player receives message: {message:?}");
            self.observer.enqueue_received(message);
        }

        fn try_receive(&self) -> Option<Self::Receive> {
            let from_player = self.observer.pop_sent();
            info!("Receiving message from player: {from_player:?}");
            from_player
        }
    }
}
