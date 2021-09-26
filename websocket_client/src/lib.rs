#![deny(clippy::all, nonstandard_style, future_incompatible)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::needless_pass_by_value,
    clippy::unused_self,
    clippy::cast_lossless
)]
use log::info;
use salt_engine::{game_agent::ClientNotifier, game_runner::GameClient, game_state::PlayerId};
use smol::net::TcpStream;
use websocket_server::{
    connection::Connection,
    messages::{FromClient, FromServer, PromptMessage},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Starts the client, using the provided GameAgent.
pub async fn start(make_agent: impl FnOnce(PlayerId) -> Box<dyn GameClient>) -> Result<()> {
    info!("Salt client starting.");
    let stream = TcpStream::connect("localhost:9000").await?;
    let (connection, _) = async_tungstenite::client_async("ws://localhost:9000", stream).await?;

    let connection = Connection::new(connection);

    handle_connection(connection, make_agent).await
}

async fn handle_connection(
    mut connection: Connection,
    make_agent: impl FnOnce(PlayerId) -> Box<dyn GameClient>,
) -> Result<()> {
    // Expect a Hello

    let my_id = match connection.recv::<FromServer>().await {
        Some(FromServer::Hello(my_id)) => my_id,
        _ => panic!("unexpected response from server"),
    };
    info!("Saw a hello - my id is: {:?}", my_id);

    let mut agent = make_agent(my_id);
    let notifier = agent.make_notifier().await;

    // Send Ready
    connection.send(FromClient::Ready).await?;

    // Expect a GameStart
    let opponent_id = match connection.recv::<FromServer>().await {
        Some(FromServer::GameStart { opponent_id }) => opponent_id,
        _ => panic!("unexpected response from server"),
    };
    info!("My opponent's ID is {:?}", opponent_id);

    // Expect the game state
    let _gamestate_view = match connection.recv::<FromServer>().await {
        Some(FromServer::State(view)) => view,
        _ => panic!("unexpected response from server"),
    };

    loop {
        // Wait for signal from server that we can send an action
        info!("Waiting for message from server...");
        let msg = connection
            .recv::<FromServer>()
            .await
            .expect("No message.  This implies the server has closed the connection.");

        match msg {
            FromServer::TurnStart => {
                handle_turn(&mut connection, agent.as_mut(), notifier.as_ref()).await?
            }
            FromServer::State(state) => agent.observe_state_update(state).await,
            FromServer::NotifyEvent(event) => notifier.notify(event).await,
            _ => panic!("expected a TurnStart message, but received: {:?}", msg),
        }
    }
}

async fn handle_turn(
    connection: &mut Connection,
    // agent: &dyn GameAgent,
    agent: &mut dyn GameClient,
    agent_notifier: &dyn ClientNotifier,
) -> Result<()> {
    // Continuously receive actions from the client, until they end their turn.
    info!("Server says my turn has started.");
    loop {
        // Wait for signal from server that we can send an action
        let msg = connection
            .recv::<FromServer>()
            .await
            .expect("failed to get a response from the server");

        match msg {
            FromServer::WaitingForAction(state) => {
                info!("Server says: waiting for action.");
                let player_action = agent.next_action(state).await;

                let is_turn_ending = player_action.is_end_turn();

                info!("Sending my action to server.");
                connection
                    .send(FromClient::ClientAction(player_action))
                    .await?;

                if is_turn_ending {
                    return Ok(());
                }
            }
            FromServer::Prompt(prompt_msg, game_state) => {
                info!("Received prompt request from server. Prompting player.");
                let prompter = agent.make_prompter().await;
                let player_input = match prompt_msg {
                    PromptMessage::PromptSlot => prompter.prompt_slot(&game_state),
                    PromptMessage::PromptCreaturePos => prompter.prompt_creature_pos(&game_state),
                    PromptMessage::PromptOpponentCreaturePos => {
                        prompter.prompt_opponent_creature_pos(&game_state)
                    }
                    PromptMessage::PromptOpponentSlot => prompter.prompt_opponent_slot(&game_state),
                    PromptMessage::PromptPlayerCreaturePos => {
                        prompter.prompt_player_creature_pos(&game_state)
                    }
                    PromptMessage::PromptPlayerSlot => prompter.prompt_player_slot(&game_state),
                };

                info!("Responding to server with prompt result.");
                connection
                    .send(FromClient::PromptResponse(player_input))
                    .await?;
            }
            FromServer::State(state) => agent.observe_state_update(state).await,
            FromServer::NotifyEvent(event) => agent_notifier.notify(event).await,
            _ => panic!("Unexpected message from server: {:?}", msg),
        }
    }
}