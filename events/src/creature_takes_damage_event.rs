pub use self::event::*;
pub use self::handler::*;

const HANDLER_NAME: &str = "CreatureTakesDamageEventHandler";

/// Module containing the event for drawing a card.
mod event {
    use super::HANDLER_NAME;
    use engine::{
        event::{Event, EventType},
        CardId, PlayerId,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreatureTakesDamageEvent {
        card_to_damage: CardId,
        damage: i16,
    }

    impl CreatureTakesDamageEvent {
        pub fn new(card_to_damage: CardId, damage: i16) -> Self {
            Self {
                card_to_damage,
                damage,
            }
        }

        pub fn card_to_damage(&self) -> CardId {
            self.card_to_damage
        }

        pub fn damage(&self) -> i16 {
            self.damage
        }
    }

    impl Event for CreatureTakesDamageEvent {
        fn event_type() -> EventType {
            EventType::new(HANDLER_NAME)
        }
    }
}

/// Module containing the handler for the draw card event.
mod handler {
    use super::HANDLER_NAME;
    use crate::{
        creature_attacks_target_event::CreatureAttacksTargetEvent,
        creature_takes_damage_event::CreatureTakesDamageEvent, HiddenInfo,
    };
    use engine::{
        event::{EventHandler, EventMessage, EventType},
        Dispatcher, FromServer, GameState,
    };
    use log::info;

    pub struct CreatureTakesDamageEventHandler;

    impl CreatureTakesDamageEventHandler {
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for CreatureTakesDamageEventHandler {
        fn default() -> Self {
            Self::new()
        }
    }

    impl EventHandler for CreatureTakesDamageEventHandler {
        fn event_type(&self) -> EventType {
            EventType::new(HANDLER_NAME)
        }

        fn handle(
            &self,
            event: &EventMessage,
            game_state: &mut GameState,
            dispatcher: &Dispatcher,
        ) {
            let unpacked_event: CreatureTakesDamageEvent = event.unpack();
            let damage = unpacked_event.damage();

            let card = game_state
                .card_mut(unpacked_event.card_to_damage())
                .expect("CardId invalid for CreatureTakesDamageEvent");

            let initial_health = card.current_health();

            if initial_health <= damage {
                info!("Target: {card:?} is destroyed");
            }

            let next_health = initial_health - damage;
            card.set_health(next_health);

            info!("Target has new health: {next_health}");

            dispatcher
                .player_a_channel()
                .send(FromServer::Event(event.clone()));
            dispatcher
                .player_b_channel()
                .send(FromServer::Event(event.clone()));
        }
    }
}
