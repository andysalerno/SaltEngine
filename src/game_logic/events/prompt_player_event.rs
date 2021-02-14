use crate::game_state::PlayerId;

use super::{Event, GameEvent};

#[derive(Debug)]
pub enum PromptFor {
    Position,
}

#[derive(Debug)]
pub struct PromptPlayerEvent {
    player_id: PlayerId,
    prompt_for: PromptFor,
}

impl PromptPlayerEvent {
    pub fn new(player_id: PlayerId, prompt_for: PromptFor) -> Self {
        Self {
            player_id,
            prompt_for,
        }
    }
}

impl Event for PromptPlayerEvent {}

impl Into<GameEvent> for PromptPlayerEvent {
    fn into(self) -> GameEvent {
        GameEvent::PromptPlayerEvent(self)
    }
}
