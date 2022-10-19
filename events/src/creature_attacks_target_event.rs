pub use self::event::*;
pub use self::handler::*;

const HANDLER_NAME: &str = "CreatureAttacksTargetEventHandler";

/// Module containing the event for drawing a card.
mod event {
    use super::HANDLER_NAME;
    use engine::{
        event::{Event, EventType},
        CardId, PlayerId,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreatureAttacksTargetEvent {
        player_id: PlayerId,
        attacker: CardId,
        target: CardId,
    }

    impl CreatureAttacksTargetEvent {
        pub fn new(player_id: PlayerId, attacker: CardId, target: CardId) -> Self {
            Self {
                player_id,
                attacker,
                target,
            }
        }

        pub fn player_id(&self) -> PlayerId {
            self.player_id
        }

        pub fn attacker(&self) -> CardId {
            self.attacker
        }

        pub fn target(&self) -> CardId {
            self.target
        }
    }

    impl Event for CreatureAttacksTargetEvent {
        fn event_type() -> EventType {
            EventType::new(HANDLER_NAME)
        }
    }
}

/// Module containing the handler for the draw card event.
mod handler {
    use super::HANDLER_NAME;
    use crate::{creature_attacks_target_event::CreatureAttacksTargetEvent, HiddenInfo};
    use engine::{
        event::{EventHandler, EventMessage, EventType},
        Dispatcher, FromServer, GameState,
    };
    use log::info;

    pub struct CreatureAttacksTargetEventHandler;

    impl CreatureAttacksTargetEventHandler {
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for CreatureAttacksTargetEventHandler {
        fn default() -> Self {
            Self::new()
        }
    }

    impl EventHandler for CreatureAttacksTargetEventHandler {
        fn event_type(&self) -> EventType {
            EventType::new(HANDLER_NAME)
        }

        fn handle(
            &self,
            event: &EventMessage,
            game_state: &mut GameState,
            _dispatcher: &Dispatcher,
        ) {
            let event: CreatureAttacksTargetEvent = event.unpack();

            let attacker_card = game_state
                .card(event.attacker())
                .expect("Expected card to exist.");
            info!("Found attacker: {attacker_card:?}");

            let damage = attacker_card.current_attack();

            let target = game_state
                .card_mut(event.target())
                .expect("Expected card to exist.");
            info!("Found target: {target:?}");

            let initial_health = target.current_health();

            if initial_health < damage {
                info!("Target: {target:?} is destroyed");
            }

            let next_health = initial_health - damage;
            target.set_health(next_health);

            info!("Target has new health: {next_health}");
        }
    }
}
