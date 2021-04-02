use crate::messages::FromClient;
use crate::messages::FromServer;
use crate::{connection::Connection, messages::PromptMessage};
use log::info;
use salt_engine::{
    game_agent::game_agent::Prompter,
    game_state::{board::BoardPos, GameStatePlayerView},
};

pub(crate) struct NewtorkPrompter {
    connection: Connection,
}

impl NewtorkPrompter {
    pub(crate) fn new(connection: Connection) -> Self {
        Self { connection }
    }

    fn send_prompt(&self, message: PromptMessage) -> BoardPos {
        info!("Prompting for slot.");

        smol::block_on(async {
            self.connection
                .send(FromServer::Prompt(message))
                .await
                .expect("failed to send prompt request");

            let response = self
                .connection
                .recv::<FromClient>()
                .await
                .expect("no response from server");

            match response {
                FromClient::PosInput(pos) => pos,
                _ => panic!("Expected PosInput from client"),
            }
        })
    }
}

impl Prompter for NewtorkPrompter {
    fn prompt_slot(&self, _game_state: &GameStatePlayerView) -> BoardPos {
        self.send_prompt(PromptMessage::PromptSlot)
    }

    fn prompt_player_slot(&self, _game_state: &GameStatePlayerView) -> BoardPos {
        self.send_prompt(PromptMessage::PromptPlayerSlot)
    }

    fn prompt_opponent_slot(&self, _game_state: &GameStatePlayerView) -> BoardPos {
        self.send_prompt(PromptMessage::PromptOpponentSlot)
    }

    fn prompt_creature_pos(&self, _game_state: &GameStatePlayerView) -> BoardPos {
        self.send_prompt(PromptMessage::PromptCreaturePos)
    }

    fn prompt_player_creature_pos(&self, _game_state: &GameStatePlayerView) -> BoardPos {
        self.send_prompt(PromptMessage::PromptPlayerCreaturePos)
    }

    fn prompt_opponent_creature_pos(&self, _game_state: &GameStatePlayerView) -> BoardPos {
        self.send_prompt(PromptMessage::PromptOpponentCreaturePos)
    }
}
