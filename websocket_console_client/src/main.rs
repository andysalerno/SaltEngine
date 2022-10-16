use engine::{
    event::{Event, EventHandler, EventMessage},
    FromClient, FromServer, GameState, MessageChannel, PlayerId,
};
use events::{CardDrawnClientEvent, HiddenInfo, PlayerStartTurnEvent};
use log::info;

mod websocket_channel;

fn main() {
    init_logger();

    let receiver = websocket_channel::connect();

    let handlers: Vec<Box<dyn EventHandler>> = Vec::new();

    // First, we expect a Hello message, with our player IDs.
    let (my_id, enemy_id) = match receiver.try_receive().expect("connection must be open.") {
        engine::FromServer::Event(_) => panic!("Expected Hello."),
        engine::FromServer::Hello(a, b) => (a, b),
    };

    let game_state = GameState::new(my_id, enemy_id);

    info!("I am: {my_id:?}. Enemy is: {enemy_id:?}");

    loop {
        info!("waiting for message...");
        let message = receiver.try_receive().unwrap();
        info!("received message: {message:?}");

        let event = match message {
            FromServer::Event(e) => e,
            _ => panic!("unexpected FromServer message"),
        };

        handle_event(event, my_id, &receiver);
    }
}

fn handle_event(
    event: EventMessage,
    my_id: PlayerId,
    receiver: &impl MessageChannel<Send = FromClient, Receive = FromServer>,
) {
    if event.event_type() == &PlayerStartTurnEvent::event_type() {
        let event: PlayerStartTurnEvent = event.unpack();
        if event.player_id() == my_id {
            info!("My turn has started!");

            info!("Sending end turn action...");
            let my_action = player_next_action();
            receiver.send(my_action);
            info!("Sent.");
        } else {
            info!("Enemy turn has started.")
        }
    } else if event.event_type() == &CardDrawnClientEvent::event_type() {
        let event: CardDrawnClientEvent = event.unpack();

        if let HiddenInfo::Visible(drawn) = event.card_drawn() {
            info!("I draw a card: {drawn:?}");
        } else {
            info!("Opponent drew a card.");
        }
    }
}

fn enemey_take_turn() {}

fn player_next_action() -> FromClient {
    FromClient::EndTurn
}

fn init_logger() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format_timestamp_millis();
    builder.init();
}
