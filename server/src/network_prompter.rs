use crate::messages::FromClient;
use crate::messages::FromServer;
use crate::{connection::Connection, messages::PromptMessage};
use log::info;
use salt_engine::{
    game_agent::Prompter,
    game_state::{board::BoardPos, GameStatePlayerView},
};

pub(crate) struct NewtorkPrompter {
    connection: Connection,
}

impl NewtorkPrompter {
    pub(crate) fn new(connection: Connection) -> Self {
        Self { connection }
    }

    fn send_prompt(&self, message: PromptMessage, game_state: &GameStatePlayerView) -> BoardPos {
        info!("Prompting for slot.");

        smol::block_on(async {
            self.connection
                .send(FromServer::Prompt(message, game_state.clone()))
                .await
                .expect("failed to send prompt request");

            let response = self
                .connection
                .recv::<FromClient>()
                .await
                .expect("no response from server");

            match response {
                FromClient::PromptResponse(pos) => pos,
                _ => panic!("Expected PromptResponse from client"),
            }
        })
    }
}

impl Prompter for NewtorkPrompter {
    fn prompt_slot(&self, game_state: &GameStatePlayerView) -> BoardPos {
        self.send_prompt(PromptMessage::PromptSlot, game_state)
    }

    fn prompt_player_slot(&self, game_state: &GameStatePlayerView) -> BoardPos {
        self.send_prompt(PromptMessage::PromptPlayerSlot, game_state)
    }

    fn prompt_opponent_slot(&self, game_state: &GameStatePlayerView) -> BoardPos {
        self.send_prompt(PromptMessage::PromptOpponentSlot, game_state)
    }

    fn prompt_creature_pos(&self, game_state: &GameStatePlayerView) -> BoardPos {
        self.send_prompt(PromptMessage::PromptCreaturePos, game_state)
    }

    fn prompt_player_creature_pos(&self, game_state: &GameStatePlayerView) -> BoardPos {
        self.send_prompt(PromptMessage::PromptPlayerCreaturePos, game_state)
    }

    fn prompt_opponent_creature_pos(&self, game_state: &GameStatePlayerView) -> BoardPos {
        self.send_prompt(PromptMessage::PromptOpponentCreaturePos, game_state)
    }
}
