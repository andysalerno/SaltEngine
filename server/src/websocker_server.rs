use async_tungstenite::tungstenite;
use futures::sink::SinkExt;
use salt_engine::game_state::PlayerId;
use server::messages::FromServer;
use server::messages::IntoJson;
use smol::{prelude::*, Async};
use std::net::{TcpListener, TcpStream};
use tungstenite::Message;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn handle_connection(stream: Async<TcpStream>) -> Result<()> {
    let mut ws_stream = async_tungstenite::accept_async(stream).await?;

    send(&mut ws_stream, FromServer::Hello(PlayerId::new()).json()).await?;

    ws_stream
        .send(Message::Text("welcome!!".to_string()))
        .await?;

    while let Some(Ok(Message::Text(t))) = ws_stream.next().await {
        println!("Saw message from client: {}", t);

        ws_stream
            .send(Message::Text("thanks for your message!!".to_string()))
            .await?;
    }

    println!("Connection closed.");

    Ok(())
}

pub fn run() -> Result<()> {
    smol::block_on(async {
        let listener = Async::<TcpListener>::bind(([127, 0, 0, 1], 9000))?;

        while let Ok((stream, addr)) = listener.accept().await {
            println!("New connection from {}", addr);

            smol::spawn(async {
                handle_connection(stream).await.unwrap();
            })
            .detach();
        }

        Ok(())
    })
}

async fn send<S, M>(stream: &mut S, message: M) -> Result<()>
where
    S: SinkExt<Message> + Unpin,
    M: Into<String>,
    <S as futures::Sink<Message>>::Error: std::error::Error + 'static,
{
    stream.send(Message::Text(message.into())).await?;
    Ok(())
}
