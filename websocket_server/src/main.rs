use engine::{event::EventHandler, Dispatcher, FromServer, GameState, MessageChannel, PlayerId};
use events::{
    DrawCardEventHandler, PlayerEndTurnEvent, PlayerEndTurnEventHandler, PlayerStartTurnEvent,
    PlayerStartTurnEventHandler, StartGameEvent, StartGameEventHandler,
};
use log::info;

mod websocket_player;

fn main() {
    init_logger();

    let (player_a, player_b) = websocket_player::accept_connections();

    let player_a_id = PlayerId::new();
    let player_b_id = PlayerId::new();
    player_a.send(FromServer::Hello(player_a_id, player_b_id));
    player_b.send(FromServer::Hello(player_b_id, player_a_id));

    info!("Both players connected. Starting game.");
    let dispatcher = {
        let handlers: Vec<Box<dyn EventHandler>> = vec![
            Box::new(DrawCardEventHandler::new()),
            Box::new(StartGameEventHandler::new()),
            Box::new(PlayerStartTurnEventHandler::new()),
            Box::new(PlayerEndTurnEventHandler::new()),
        ];
        let player_a = Box::new(player_a);
        let player_b = Box::new(player_b);
        Dispatcher::new(handlers, player_a, player_b)
    };

    // First, dispatch the StartGame event. Both players draw cards to prepare for gameplay.
    let mut game_state = GameState::new(player_a_id, player_b_id);
    dispatcher.dispatch(&StartGameEvent::new().into(), &mut game_state);

    // Then, keep getting player input until the game is over.
    while !game_state.is_game_over() {
        player_take_turn(&mut game_state, &dispatcher);
    }
}

fn player_take_turn(game_state: &mut GameState, dispatcher: &Dispatcher) {
    let player_turn = game_state.cur_player_turn();
    let event = PlayerStartTurnEvent::new(player_turn);
    dispatcher.dispatch(&event.into(), game_state);

    let message = if player_turn == game_state.player_id_a() {
        dispatcher.player_a().try_receive()
    } else if player_turn == game_state.player_id_b() {
        dispatcher.player_b().try_receive()
    } else {
        panic!("Unknown player id.")
    };

    info!("Received message: {message:?}");

    let message = message.unwrap();

    match message {
        engine::FromClient::EndTurn => {
            let event = PlayerEndTurnEvent::new(player_turn);
            dispatcher.dispatch(&event.into(), game_state);
        }
    }
}

fn init_logger() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format_timestamp_millis();
    builder.init();
}
