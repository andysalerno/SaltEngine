#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Mutex};

    use engine::{
        deck::Deck, event::EventHandler, CardDefinition, ClientChannel, Dispatcher, FromClient,
        FromServer, GameState, MessageChannel, PlayerId,
    };
    use events::{DrawCardEventHandler, StartGameEvent, StartGameEventHandler};
    use log::info;

    #[test]
    fn on_game_start_expects_cards_drawn() {
        init_logger();

        let player_a_id = PlayerId::new();
        let player_b_id = PlayerId::new();

        // Keep track of messages received by both players.
        let a_messages = Rc::new(Mutex::new(Vec::new()));
        let b_messages = Rc::new(Mutex::new(Vec::new()));

        let dispatcher = {
            let handlers: Vec<Box<dyn EventHandler>> = vec![
                Box::new(DrawCardEventHandler::new()),
                Box::new(StartGameEventHandler::new()),
            ];

            let player_a_channel = {
                let mut client = DummyClient::new();
                let messages = Rc::clone(&a_messages);
                client.on_push_message(move |message| {
                    info!("player_a saw message: {message:?}");
                    messages.lock().unwrap().push(message);
                });

                ClientChannel::new(player_a_id, Box::new(client))
            };

            let player_b_channel = {
                let mut client = DummyClient::new();
                let messages = Rc::clone(&b_messages);
                client.on_push_message(move |message| {
                    info!("player_b saw message: {message:?}");
                    messages.lock().unwrap().push(message);
                });

                ClientChannel::new(player_b_id, Box::new(client))
            };

            Dispatcher::new(handlers, player_a_channel, player_b_channel)
        };

        let mut game_state = {
            let mut builder = GameState::builder(player_a_id, player_b_id);
            builder
                .with_player_a_deck(make_deck())
                .with_player_b_deck(make_deck());
            builder.build()
        };

        // Pre-assert condition
        let hand_len = game_state.hand(player_a_id).len();
        assert!(hand_len == 0);

        dispatcher.dispatch(&StartGameEvent::new().into(), &mut game_state);

        let hand_len = game_state.hand(player_a_id).len();

        assert!(hand_len == 6);
    }

    #[test]
    fn on_creature_attack_expects_take_damage() {
        init_logger();

        let handlers: Vec<Box<dyn EventHandler>> = vec![
            Box::new(DrawCardEventHandler::new()),
            Box::new(StartGameEventHandler::new()),
        ];

        let player_a_id = PlayerId::new();
        let player_b_id = PlayerId::new();

        let dispatcher = make_dispatcher(player_a_id, player_b_id, handlers);

        let mut game_state = {
            let mut builder = GameState::builder(player_a_id, player_b_id);
            builder
                .with_player_a_deck(make_deck())
                .with_player_b_deck(make_deck());
            builder.build()
        };

        dispatcher.dispatch(&StartGameEvent::new().into(), &mut game_state);
    }

    fn make_dispatcher(
        player_a_id: PlayerId,
        player_b_id: PlayerId,
        handlers: impl IntoIterator<Item = Box<dyn EventHandler>>,
    ) -> Dispatcher {
        let a_messages = Rc::new(Mutex::new(Vec::new()));
        let b_messages = Rc::new(Mutex::new(Vec::new()));

        let handlers = handlers.into_iter().collect();

        let player_a_channel = {
            let mut client = DummyClient::new();
            let messages = Rc::clone(&a_messages);
            client.on_push_message(move |message| {
                info!("player_a saw message: {message:?}");
                messages.lock().unwrap().push(message);
            });

            ClientChannel::new(player_a_id, Box::new(client))
        };

        let player_b_channel = {
            let mut client = DummyClient::new();
            let messages = Rc::clone(&b_messages);
            client.on_push_message(move |message| {
                info!("player_b saw message: {message:?}");
                messages.lock().unwrap().push(message);
            });

            ClientChannel::new(player_b_id, Box::new(client))
        };

        Dispatcher::new(handlers, player_a_channel, player_b_channel)
    }

    fn make_deck() -> Deck {
        let mut deck = Deck::new_empty();

        let mut builder = CardDefinition::builder();
        builder.title("test_card");

        for _ in 0..10 {
            deck.add_card_to_bottom(builder.build());
        }

        deck
    }

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    struct DummyClient<'a> {
        on_push_message: Box<dyn Fn(FromServer) + 'a>,
    }

    impl<'a> DummyClient<'a> {
        fn new() -> Self {
            Self {
                on_push_message: Box::new(|_| {}),
            }
        }

        fn on_push_message<TFn>(&mut self, action: TFn)
        where
            TFn: Fn(FromServer) + 'a,
        {
            self.on_push_message = Box::new(action);
        }
    }

    impl<'a> MessageChannel for DummyClient<'a> {
        type Send = FromServer;
        type Receive = FromClient;
        fn send(&self, message: FromServer) {
            (self.on_push_message)(message);
        }

        fn try_receive(&self) -> Option<Self::Receive> {
            None
        }
    }
}
