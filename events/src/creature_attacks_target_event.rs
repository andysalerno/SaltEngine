pub use self::event::*;
pub use self::handler::*;

const HANDLER_NAME: &str = "CreatureAttacksTargetEvent";

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
    use crate::{
        creature_attacks_target_event::CreatureAttacksTargetEvent,
        creature_takes_damage_event::CreatureTakesDamageEvent, HiddenInfo,
    };
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
            dispatcher: &Dispatcher,
        ) {
            let unpacked_event: CreatureAttacksTargetEvent = event.unpack();

            let attacker_card = game_state
                .card(unpacked_event.attacker())
                .expect("Expected card to exist.");
            info!("Found attacker: {attacker_card:?}");

            let damage = attacker_card.current_attack();

            let target = game_state
                .card_mut(unpacked_event.target())
                .expect("Expected card to exist.");
            info!("Found target: {target:?}");

            let target_attack = target.current_attack();

            // First, show players this event, before continuing the chain.
            dispatcher
                .player_a_channel()
                .send(FromServer::Event(event.clone()));
            dispatcher
                .player_b_channel()
                .send(FromServer::Event(event.clone()));

            // Target takes damage from attacker
            let damage_event = CreatureTakesDamageEvent::new(unpacked_event.target(), damage);
            dispatcher.dispatch(damage_event, game_state);

            // Attacker takes damage from target
            let damage_event =
                CreatureTakesDamageEvent::new(unpacked_event.attacker(), target_attack);
            dispatcher.dispatch(damage_event, game_state);
        }
    }
}
