use crate::game_state::GameState;

use super::is::Downcast;

pub trait Event: Downcast {}

pub trait EventHandler {
    type Event: Event;
    fn handle(&self, event: &Self::Event, game_state: &mut GameState);
}

// fn testing() {
//     let mut handlers = Vec::<Box<dyn EventHandler>>::new();
//     handlers.push(Box::new(AttackHandler));
//     handlers.push(Box::new(PlayCardHandler));

//     let some_event: Box<dyn Event> = Box::new(AttackEvent);

//     let handler = handlers.iter().filter(|h| h.can_handle(&some_event)).next();

//     // for handler in handlers {
//     //     if handler.can_handle(&some_event) {
//     //         handler.handle(some_event.as_ref());
//     //     }
//     // }
// }
