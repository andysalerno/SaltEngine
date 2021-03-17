use salt_engine::game_state::PlayerId;
use server::messages::{Connection, FromClient, FromServer};
use smol::Async;
use std::net::{TcpListener, TcpStream};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn handle_connection(stream: Async<TcpStream>) -> Result<()> {
    let ws_stream = async_tungstenite::accept_async(stream).await?;

    let mut connection = Connection::new(ws_stream);
    connection.send(FromServer::Hello(PlayerId::new())).await?;

    let message = connection.recv::<FromClient>().await;
    println!("Received: {:?}", message);

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
