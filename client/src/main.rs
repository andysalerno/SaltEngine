use server::{
    connection::Connection,
    messages::{FromClient, FromServer},
};
use smol::net::TcpStream;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    smol::block_on(async {
        let stream = TcpStream::connect("localhost:9000").await?;
        let (connection, _) =
            async_tungstenite::client_async("ws://localhost:9000", stream).await?;

        let connection = Connection::new(connection);

        handle_connection(connection).await
    })
}

async fn handle_connection(mut connection: Connection) -> Result<()> {
    // Expect a Hello
    let my_id = match connection.recv::<FromServer>().await {
        Some(FromServer::Hello(my_id)) => my_id,
        _ => panic!("unexpected response from server"),
    };
    println!("Saw a hello - my id is: {:?}", my_id);

    // Send Ready
    connection.send(FromClient::Ready).await?;

    // Expect a GameStart
    let opponent_id = match connection.recv::<FromServer>().await {
        Some(FromServer::GameStart { opponent_id }) => opponent_id,
        _ => panic!("unexpected response from server"),
    };
    println!("My opponent's ID is {:?}", opponent_id);

    // Expect the game state

    let gamestate_view = match connection.recv::<FromServer>().await {
        Some(FromServer::State(view)) => view,
        _ => panic!("unexpected response from server"),
    };
    println!("My starting hand is: {:?}", gamestate_view.hand());

    Ok(())
}
