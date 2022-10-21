mod creature_attacks_target_event;
mod creature_placed_on_board_event;
mod draw_card_event;
mod player_end_turn_event;
mod player_start_turn_event;
mod start_game_event;

pub use creature_attacks_target_event::{
    CreatureAttacksTargetEvent, CreatureAttacksTargetEventHandler,
};
pub use creature_placed_on_board_event::{
    CreaturePlacedOnBoardEvent, CreaturePlacedOnBoardEventHandler,
};
pub use draw_card_event::{CardDrawnClientEvent, DrawCardEvent, DrawCardEventHandler};
pub use player_end_turn_event::{PlayerEndTurnEvent, PlayerEndTurnEventHandler};
pub use player_start_turn_event::{PlayerStartTurnEvent, PlayerStartTurnEventHandler};
use serde::{Deserialize, Serialize};
pub use start_game_event::{StartGameEvent, StartGameEventHandler};

/// Information on an event that may be visible to one player but hidden to another.
/// Example: a card draw event for player A will hide the drawn card from player B.
#[derive(Clone, Serialize, Debug, Deserialize)]
pub enum HiddenInfo<T> {
    Visible(T),
    Hidden,
}

impl<T> HiddenInfo<T> {
    pub fn is_hidden(&self) -> bool {
        matches!(self, Self::Hidden)
    }

    pub fn is_visible(&self) -> bool {
        !self.is_hidden()
    }
}
