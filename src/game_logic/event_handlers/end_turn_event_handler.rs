use crate::game_logic::{
    event::{Event, EventHandler},
    is::Is,
};

pub struct EndTurnEventHandler;

impl EventHandler for EndTurnEventHandler {
    fn can_handle(&self, event: &Box<dyn Event>) -> bool {
        event.is::<EndTurnEventHandler>()
    }

    fn handle(&self, event: &dyn Event) {
        todo!()
    }
}
