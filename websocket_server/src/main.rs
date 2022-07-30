use engine::{event::EventHandler, ClientChannel, Dispatcher, FromClient, GameState, PlayerId};
use events::{DrawCardEventHandler, StartGameEvent, StartGameEventHandler};

struct DummyClient;

impl ClientChannel for DummyClient {
    fn push_message(&self, message: &engine::event::EventMessage) {
        //
    }

    fn try_receive_message(&self) -> Option<FromClient> {
        None
    }
}

fn main() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format_timestamp_millis();
    builder.init();

    let handlers: Vec<Box<dyn EventHandler>> = vec![
        Box::new(DrawCardEventHandler::new()),
        Box::new(StartGameEventHandler::new()),
    ];
    let player_a = Box::new(DummyClient);
    let player_b = Box::new(DummyClient);
    let dispatcher = Dispatcher::new(handlers, player_a, player_b);

    let event = StartGameEvent::new();

    let mut game_state = GameState::new(PlayerId::new(), PlayerId::new());

    dispatcher.dispatch(&event.into(), &mut game_state);
}
