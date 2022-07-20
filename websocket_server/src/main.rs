use engine::{event::EventHandler, Dispatcher, GameState, PlayerId};
use events::{DrawCardEventHandler, StartGameEvent, StartGameEventHandler};

fn main() {
    env_logger::init();

    let handlers: Vec<Box<dyn EventHandler>> = vec![
        Box::new(DrawCardEventHandler::new()),
        Box::new(StartGameEventHandler::new()),
    ];
    let dispatcher = Dispatcher::new(handlers);

    let event = StartGameEvent::new();

    let mut game_state = GameState::new(PlayerId::new(), PlayerId::new());

    dispatcher.dispatch(&event.into(), &mut game_state);
}
