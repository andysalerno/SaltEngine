use crate::game_logic::{
    event::{Event, EventHandler},
    events::AttackEvent,
    is::Is,
};

pub struct AttackEventHandler;

impl EventHandler for AttackEventHandler {
    fn can_handle(&self, event: &Box<dyn Event>) -> bool {
        event.is::<AttackEvent>()
    }

    fn handle(&self, event: &dyn Event) {
        todo!()
    }
}
