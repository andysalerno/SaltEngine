use server::{
    connection::Connection,
    messages::{FromClient, FromServer},
};
use smol::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    smol::block_on(async {
        let stream = TcpStream::connect("localhost:9000").await?;
        let (connection, _) =
            async_tungstenite::client_async("ws://localhost:9000", stream).await?;

        let connection = Connection::new(connection);

        handle_connection(connection).await
    })
}

async fn handle_connection(mut connection: Connection) -> Result<(), Box<dyn std::error::Error>> {
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
