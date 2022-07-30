#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Mutex};

    use engine::{
        event::{EventHandler, EventMessage},
        ClientChannel, Dispatcher, GameState, PlayerId,
    };
    use events::{DrawCardEventHandler, StartGameEvent, StartGameEventHandler};
    use log::info;

    struct DummyClient<'a> {
        on_push_message: Box<dyn Fn(EventMessage) + 'a>,
    }

    impl<'a> DummyClient<'a> {
        fn new() -> Self {
            Self {
                on_push_message: Box::new(|_| {}),
            }
        }

        fn on_push_message<TFn>(&mut self, action: TFn)
        where
            TFn: Fn(EventMessage) + 'a,
        {
            self.on_push_message = Box::new(action);
        }
    }

    impl<'a> ClientChannel for DummyClient<'a> {
        fn push_message(&self, message: &EventMessage) {
            (self.on_push_message)(message.clone());
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
            let messages = Rc::clone(&a_messages);
            player_a.on_push_message(move |message| {
                info!("player_a saw message: {message:?}");
                messages.lock().unwrap().push(message);
            });

            let mut player_b = Box::new(DummyClient::new());
            let messages = Rc::clone(&b_messages);
            player_b.on_push_message(move |message| {
                info!("player_b saw message: {message:?}");
                messages.lock().unwrap().push(message);
            });

            Dispatcher::new(handlers, player_a, player_b)
        };

        let mut game_state = GameState::new(PlayerId::new(), PlayerId::new());

        dispatcher.dispatch(&StartGameEvent::new().into(), &mut game_state);

        let l = a_messages.lock().unwrap().len();
        info!("final count of messages for player_a: {l}");
    }
}
