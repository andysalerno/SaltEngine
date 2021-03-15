use std::net::{TcpListener, TcpStream};
use std::pin::Pin;
use std::task::{Context, Poll};
//use anyhow::{Context as _, Result};
//use async_native_tls::{Identity, TlsAcceptor, TlsStream};
use async_tungstenite::{tungstenite, WebSocketStream};
use futures::sink::{Sink, SinkExt};
use smol::{future, prelude::*, Async};
use tungstenite::Message;

type WsStream = WebSocketStream<Async<TcpStream>>;
type E = async_tungstenite::tungstenite::Error;
type Result<T> = std::result::Result<T, E>;

/// Echoes messages from the client back to it.
async fn echo(mut stream: WsStream) -> Result<()> {
    let msg = stream.next().await.unwrap()?;
    stream.send(Message::text(msg.to_string())).await?;
    Ok(())
}

/// Listens for incoming connections and serves them.
async fn listen(listener: Async<TcpListener>) -> Result<()> {
    let host = format!("ws://{}", listener.get_ref().local_addr()?);
    println!("Listening on {}", host);

    loop {
        // Accept the next connection.
        let (stream, _) = listener.accept().await?;
        println!("Accepted client: {}", stream.get_ref().peer_addr()?);

        let stream: WsStream = async_tungstenite::accept_async(stream).await?;
        smol::spawn(echo(stream)).detach();
    }
}

async fn handle_connection(stream: Async<TcpStream>) -> Result<()> {
    let mut ws_stream = async_tungstenite::accept_async(stream).await?;

    // Clients are only allowed to send text messages at this stage.
    // If they do anything else, then just disconnect.
    ws_stream
        .send(Message::Text("welcome!!".to_string()))
        .await?;

    while let Some(Ok(Message::Text(t))) = ws_stream.next().await {
        println!("Saw message from client: {}", t);

        ws_stream
            .send(Message::Text("thanks for your message!!".to_string()))
            .await?;
    }

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
