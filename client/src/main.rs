use async_tungstenite::tungstenite::{self, client::connect_with_config, Message};
use futures::SinkExt;
use salt_engine::id::Id;
use server::messages::{from_client::*, from_server::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    smol::block_on(async {
        let (mut connection, _) =
            async_tungstenite::async_std::connect_async("ws://localhost:9000")
                .await
                .expect("failed to connect");

        connection
            .send(Message::Text("yoooo!!".to_string()))
            .await
            .expect("failed to send message");
    });
    Ok(())
}
