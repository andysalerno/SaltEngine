#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Mutex};

    use engine::{
        deck::Deck, event::EventHandler, Card, CardDefinition, ClientChannel, Dispatcher,
        FromClient, FromServer, GamePos, GameState, MessageChannel, PlayerId,
    };
    use events::{
        CardDrawnClientEvent, CreatureAttacksTargetEvent, CreatureAttacksTargetEventHandler,
        DrawCardEvent, DrawCardEventHandler, PlayerStartTurnEvent, PlayerStartTurnEventHandler,
        StartGameEvent, StartGameEventHandler,
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
            Box::new(DrawCardEventHandler::new()),
            Box::new(PlayerStartTurnEventHandler::new()),
            Box::new(StartGameEventHandler::new()),
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

        // 1. Start game
        dispatcher.dispatch(StartGameEvent::new(), &mut game_state);

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

        // 2. PlayerA turn starts
        dispatcher.dispatch(PlayerStartTurnEvent::new(player_a_id), &mut game_state);

        // 3. Receive action from PlayerA
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
            deck.add_card_to_bottom(builder.build());
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
        received_messages: Rc<Mutex<Vec<FromServer>>>,
        messages_to_send: Rc<Mutex<Vec<FromClient>>>,
    }

    impl ClientObserver {
        fn new() -> Self {
            Self {
                received_messages: Rc::new(Mutex::new(Vec::new())),
                messages_to_send: Rc::new(Mutex::new(Vec::new())),
            }
        }

        fn push_received(&self, message: FromServer) {
            self.received_messages.lock().unwrap().push(message);
        }

        fn pop_received(&self) -> Option<FromServer> {
            self.received_messages.lock().unwrap().pop()
        }

        fn push_sent(&self, message: FromClient) {
            self.messages_to_send.lock().unwrap().push(message);
        }

        fn pop_sent(&self) -> Option<FromClient> {
            self.messages_to_send.lock().unwrap().pop()
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
            self.observer.push_received(message);
        }

        fn try_receive(&self) -> Option<Self::Receive> {
            let from_player = self.observer.pop_sent();
            info!("Receiving message from player: {from_player:?}");
            from_player
        }
    }
}
