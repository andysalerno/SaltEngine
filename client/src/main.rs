use async_tungstenite::{tungstenite::Message, WebSocketStream};
use futures::{stream, Sink, SinkExt, Stream, StreamExt};
use salt_engine::id::Id;
use server::messages::{FromClient, FromJson, FromServer, GameMessage, IntoJson};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    smol::block_on(async {
        let (mut connection, _) =
            async_tungstenite::async_std::connect_async("ws://localhost:9000").await?;

        send(&mut connection, FromClient::JoinGame.json()).await?;

        let response = connection.next().await.expect("Connection died")?;
        println!("Got response: {:?}", response);
        let response: FromServer = response.from_json();

        println!("Got response: {:?}", response);

        Ok(())
    })
}

async fn send<S, M>(stream: &mut S, message: M) -> Result<(), Box<dyn std::error::Error>>
where
    S: SinkExt<Message> + Unpin,
    M: Into<String>,
    <S as futures::Sink<Message>>::Error: std::error::Error + 'static,
{
    stream.send(Message::Text(message.into())).await?;
    Ok(())
}
