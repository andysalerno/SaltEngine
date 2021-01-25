use super::is::Downcast;

pub trait Event: Downcast {}

pub trait EventHandler {
    fn can_handle(&self, event: &Box<dyn Event>) -> bool;
    fn handle(&self, event: &dyn Event);
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
