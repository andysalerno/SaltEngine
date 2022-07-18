use engine::{event::EventHandler, Dispatcher};
use events::{DrawCardEvent, DrawCardEventHandler};

fn main() {
    env_logger::init();

    let handlers: Vec<Box<dyn EventHandler>> = vec![Box::new(DrawCardEventHandler::new())];
    let mut dispatcher = Dispatcher::new(handlers);

    let event = DrawCardEvent::new();

    dispatcher.dispatch(&event.into());
}
