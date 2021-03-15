use async_tungstenite::tungstenite;
use futures::sink::SinkExt;
use smol::{prelude::*, Async};
use std::net::{TcpListener, TcpStream};
use tungstenite::Message;

type E = async_tungstenite::tungstenite::Error;
type Result<T> = std::result::Result<T, E>;

async fn handle_connection(stream: Async<TcpStream>) -> Result<()> {
    let mut ws_stream = async_tungstenite::accept_async(stream).await?;

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
