use server::messages::{Connection, FromClient, FromServer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    smol::block_on(async {
        let (connection, _) =
            async_tungstenite::async_std::connect_async("ws://localhost:9000").await?;

        let mut connection = Connection::new(connection);

        // Expect a Hello
        let response = connection.recv::<FromServer>().await;
        println!("response: {:?}", response);

        // Send a JoinGame
        connection.send(FromClient::JoinGame).await?;

        // Expect a GameId
        let response = connection.recv::<FromServer>().await;
        println!("response: {:?}", response);

        Ok(())
    })
}
