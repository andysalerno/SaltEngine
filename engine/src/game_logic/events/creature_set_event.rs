use super::{Event, VisualEvent};
use crate::game_state::game_state::GameState;
use protocol::entities::{BoardPos, CreatureInstanceId, PlayerId};

#[derive(Debug)]
pub struct CreatureSetEvent {
    player_id: PlayerId,
    card_id: CreatureInstanceId,
    target_position: BoardPos,
}

impl CreatureSetEvent {
    #[must_use]
    pub fn new(
        player_id: PlayerId,
        card_id: CreatureInstanceId,
        target_position: BoardPos,
    ) -> Self {
        Self {
            player_id,
            card_id,
            target_position,
        }
    }

    #[must_use]
    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    #[must_use]
    pub fn card_id(&self) -> CreatureInstanceId {
        self.card_id
    }

    #[must_use]
    pub fn target_position(&self) -> BoardPos {
        self.target_position
    }
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct CreatureSetClientEvent {
//     pub player_id: PlayerId,
//     pub card_id: CreatureInstanceId,
//     pub pos: BoardPos,
//     pub card: UnitCardInstancePlayerView,
// }

impl Event for CreatureSetEvent {
    fn maybe_client_event(
        &self,
        player_id: PlayerId,
        game_state: &GameState,
    ) -> Option<VisualEvent> {
        // let view = game_state
        //     .board()
        //     .creature_instance_all(self.card_id())
        //     .player_view(self.player_id); // todo: this is wrong, since both players get the owning player's view

        // let event = CreatureSetClientEvent {
        //     player_id: self.player_id,
        //     card_id: self.card_id,
        //     pos: self.target_position,
        //     card: view,
        // };

        // Some(ClientEventView::UnitSet(event))
        None
    }
}
