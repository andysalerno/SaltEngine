use engine::{event::EventHandler, Dispatcher, GameState, PlayerId};
use events::{DrawCardEvent, DrawCardEventHandler};

fn main() {
    env_logger::init();

    let handlers: Vec<Box<dyn EventHandler>> = vec![Box::new(DrawCardEventHandler::new())];
    let mut dispatcher = Dispatcher::new(handlers);

    let event = DrawCardEvent::new();

    let mut game_state = GameState::new(PlayerId::new(), PlayerId::new());

    dispatcher.dispatch(&event.into(), &mut game_state);
}
