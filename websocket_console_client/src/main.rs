use log::info;

fn main() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format_timestamp_millis();
    builder.init();

    info!("Connecting...");
    let mut socket = loop {
        match tungstenite::connect("ws://localhost:9001") {
            Ok((socket, _)) => break socket,
            _ => continue,
        }
    };
    info!("Connected.");

    loop {
        info!("Trying to receive a message...");
        let received = socket.read_message().unwrap();
        info!("Received message: {received:?}");
    }
}
