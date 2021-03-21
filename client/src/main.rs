use futures::{AsyncRead, AsyncWrite};
use server::messages::{Connection, FromClient, FromServer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    smol::block_on(async {
        let (connection, _) =
            async_tungstenite::async_std::connect_async("ws://localhost:9000").await?;

        let connection = Connection::new(connection);

        handle_connection(connection).await
    })
}

async fn handle_connection<S>(
    mut connection: Connection<S>,
) -> Result<(), Box<dyn std::error::Error>>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    // Expect a Hello
    let my_id = match connection.recv::<FromServer>().await {
        Some(FromServer::Hello(my_id)) => my_id,
        _ => panic!("unexpected response from server"),
    };
    println!("Saw a hello - my id is: {:?}", my_id);

    // Send a JoinGame
    connection.send(FromClient::JoinGame).await?;

    // Expect a GameId
    let response = connection.recv::<FromServer>().await;
    println!("response: {:?}", response);

    // Expect the game state
    let response = connection.recv::<FromServer>().await;
    println!("response: {:?}", response);

    Ok(())
}
