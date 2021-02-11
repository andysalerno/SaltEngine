use crate::game_state::{board::BoardPos, PlayerId, UnitCardInstanceId};

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct SummonCreatureFromHandEvent {
    player_id: PlayerId,
    board_pos: BoardPos,
    hand_card_id: UnitCardInstanceId,
}

impl SummonCreatureFromHandEvent {
    pub fn new(player_id: PlayerId, board_pos: BoardPos, hand_card_id: UnitCardInstanceId) -> Self {
        Self {
            player_id,
            board_pos,
            hand_card_id,
        }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn board_pos(&self) -> BoardPos {
        self.board_pos
    }

    pub fn hand_card_id(&self) -> UnitCardInstanceId {
        self.hand_card_id
    }
}

impl Event for SummonCreatureFromHandEvent {}

impl Into<GameEvent> for SummonCreatureFromHandEvent {
    fn into(self) -> GameEvent {
        GameEvent::SummonCreatureFromHand(self)
    }
}
