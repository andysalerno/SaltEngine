use log::info;
use salt_engine::{
    game_agent::game_agent::Prompter,
    game_state::{board::BoardPos, GameStatePlayerView},
};

pub(crate) struct NewtorkPrompter;

impl NewtorkPrompter {
    pub fn new() -> Self {
        NewtorkPrompter
    }
}

impl Prompter for NewtorkPrompter {
    fn prompt_slot(&self, game_state: &GameStatePlayerView) -> BoardPos {
        info!("Prompting for slot.");
        todo!()
    }

    fn prompt_player_slot(&self, game_state: &GameStatePlayerView) -> BoardPos {
        info!("Prompting for player slot.");
        todo!()
    }

    fn prompt_opponent_slot(&self, game_state: &GameStatePlayerView) -> BoardPos {
        info!("Prompting for opponent slot.");
        todo!()
    }

    fn prompt_creature_pos(&self, game_state: &GameStatePlayerView) -> BoardPos {
        info!("Prompting for creature pos.");
        todo!()
    }

    fn prompt_player_creature_pos(&self, game_state: &GameStatePlayerView) -> BoardPos {
        info!("Prompting for player creature pos.");
        todo!()
    }

    fn prompt_opponent_creature_pos(&self, game_state: &GameStatePlayerView) -> BoardPos {
        info!("Prompting for opponent creature pos.");
        todo!()
    }
}
