#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Mutex};

    use engine::{
        event::{EventHandler, EventMessage},
        ClientChannel, Dispatcher, GameState, PlayerId,
    };
    use events::{DrawCardEventHandler, StartGameEvent, StartGameEventHandler};

    struct DummyClient<'a> {
        observed_events: Mutex<Vec<EventMessage>>,
        on_push_message: Box<dyn FnMut(EventMessage) + 'a>,
    }

    impl<'a> DummyClient<'a> {
        fn new() -> Self {
            Self {
                observed_events: Mutex::new(Vec::new()),
                on_push_message: Box::new(|_| {}),
            }
        }

        fn on_push_message<TFn>(&mut self, action: TFn)
        where
            TFn: FnMut(EventMessage) + 'a,
        {
            // self.on_push_message(Box::new(action))
            self.on_push_message = Box::new(action);
        }

        fn saw_events(&self) -> Vec<EventMessage> {
            self.observed_events.lock().unwrap().clone()
        }
    }

    impl<'a> ClientChannel for DummyClient<'a> {
        fn push_message(&self, message: &EventMessage) {
            let mut v = self.observed_events.lock().unwrap();
            v.push(message.clone());
        }

        fn try_receive_message(&self) -> Option<engine::FromClient> {
            None
        }
    }

    #[test]
    fn demo() {
        let mut builder = env_logger::Builder::from_default_env();
        builder.is_test(true).format_timestamp_millis().init();

        // Keep track of messages received by both players.
        let a_messages = Rc::new(Mutex::new(Vec::new()));
        let b_messages = Rc::new(Mutex::new(Vec::new()));

        let dispatcher = {
            let handlers: Vec<Box<dyn EventHandler>> = vec![
                Box::new(DrawCardEventHandler::new()),
                Box::new(StartGameEventHandler::new()),
            ];

            let mut player_a = Box::new(DummyClient::new());
            player_a.on_push_message(move |message| {
                a_messages.lock().unwrap().push(message);
            });

            let mut player_b = Box::new(DummyClient::new());
            player_b.on_push_message(move |message| {
                b_messages.lock().unwrap().push(message);
            });

            Dispatcher::new(handlers, player_a, player_b)
        };

        let event = StartGameEvent::new();

        let mut game_state = GameState::new(PlayerId::new(), PlayerId::new());

        dispatcher.dispatch(&event.into(), &mut game_state);
    }
}
