use crate::connection::Connection;
use protocol::{
    entities::{BoardPos, PlayerId, RowId},
    from_server::PromptMessage,
};
// use salt_engine::{game_agent::Prompter, game_state::GameStatePlayerView};
use salt_engine::game_agent::Prompter;

pub(crate) struct NewtorkPrompter {
    connection: Connection,
}

impl NewtorkPrompter {
    pub(crate) fn new(connection: Connection) -> Self {
        Self { connection }
    }

    fn send_prompt(&self, message: PromptMessage) -> BoardPos {
        // let player_a = game_state.player_id();
        let player_a = PlayerId::new();

        BoardPos::new(player_a, RowId::BackRow, 0)

        // info!("Prompting for slot.");

        // smol::block_on(async {
        //     self.connection
        //         .send(FromServer::Prompt(message, game_state.clone()))
        //         .await
        //         .expect("failed to send prompt request");

        //     let response = self
        //         .connection
        //         .recv::<FromClient>()
        //         .await
        //         .expect("no response from server");

        //     match response {
        //         FromClient::PromptResponse(pos) => pos,
        //         _ => panic!("Expected PromptResponse from client"),
        //     }
        // })
    }
}

impl Prompter for NewtorkPrompter {
    fn prompt_slot(&self) -> BoardPos {
        self.send_prompt(PromptMessage::PromptSlot)
    }

    fn prompt_player_slot(&self) -> BoardPos {
        self.send_prompt(PromptMessage::PromptPlayerSlot)
    }

    fn prompt_opponent_slot(&self) -> BoardPos {
        self.send_prompt(PromptMessage::PromptOpponentSlot)
    }

    fn prompt_creature_pos(&self) -> BoardPos {
        self.send_prompt(PromptMessage::PromptCreaturePos)
    }

    fn prompt_player_creature_pos(&self) -> BoardPos {
        self.send_prompt(PromptMessage::PromptPlayerCreaturePos)
    }

    fn prompt_opponent_creature_pos(&self) -> BoardPos {
        self.send_prompt(PromptMessage::PromptOpponentCreaturePos)
    }
}
