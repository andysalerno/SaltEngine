pub use self::event::*;
pub use self::handler::*;

const HANDLER_NAME: &str = "PlayerSummonsCreatureEvent";

mod event {
    use super::HANDLER_NAME;
    use engine::{
        event::{Event, EventType},
        CardId, GamePos, PlayerId,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PlayerSummonsCreatureEvent {
        player_id: PlayerId,
        card_id: CardId,
        target_pos: GamePos,
    }

    impl PlayerSummonsCreatureEvent {
        pub fn new(player_id: PlayerId, card_id: CardId, target_pos: GamePos) -> Self {
            Self {
                player_id,
                card_id,
                target_pos,
            }
        }

        pub fn player_id(&self) -> PlayerId {
            self.player_id
        }

        pub fn card_id(&self) -> CardId {
            self.card_id
        }

        pub fn target_pos(&self) -> GamePos {
            self.target_pos
        }
    }

    impl Event for PlayerSummonsCreatureEvent {
        fn event_type() -> EventType {
            EventType::new(HANDLER_NAME)
        }
    }
}

mod handler {
    use super::{PlayerSummonsCreatureEvent, HANDLER_NAME};
    use engine::{
        event::{EventHandler, EventMessage, EventType},
        Dispatcher, FromServer, GameState,
    };
    use log::{error, info};

    pub struct PlayerSummonsCreatureEventHandler;

    impl PlayerSummonsCreatureEventHandler {
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for PlayerSummonsCreatureEventHandler {
        fn default() -> Self {
            Self::new()
        }
    }

    impl EventHandler for PlayerSummonsCreatureEventHandler {
        fn event_type(&self) -> EventType {
            EventType::new(HANDLER_NAME)
        }

        fn handle(
            &self,
            event: &EventMessage,
            game_state: &mut GameState,
            dispatcher: &Dispatcher,
        ) {
            let unpacked: PlayerSummonsCreatureEvent = event.unpack();
            let card_id = unpacked.card_id();
            let pos = unpacked.target_pos();

            let card = game_state
                .hand_mut(unpacked.player_id())
                .take_card(card_id)
                .expect("Attempt to summon card, but ID did not match card in hand.");

            info!("Card taken from player's hand: {card_id:?}");

            game_state.set_card_at_pos(pos, card);

            info!("Card placed on board at position: {pos:?}");

            dispatcher
                .player_a_channel()
                .send(FromServer::Event(event.clone()));
            dispatcher
                .player_b_channel()
                .send(FromServer::Event(event.clone()));
        }
    }
}
