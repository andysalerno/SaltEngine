mod draw_card_event;
mod player_end_turn_event;
mod player_start_turn_event;
mod start_game_event;

pub use draw_card_event::{DrawCardEvent, DrawCardEventHandler};
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
