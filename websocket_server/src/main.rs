use engine::{event::EventHandler, Dispatcher, GameState, PlayerId};
use events::{
    DrawCardEventHandler, PlayerStartTurnEvent, PlayerStartTurnEventHandler, StartGameEvent,
    StartGameEventHandler,
};
use log::info;

mod websocket_player;

fn main() {
    init_logger();

    let (player_a, player_b) = websocket_player::accept_connections();

    info!("Both players connected. Starting game.");
    let dispatcher = {
        let handlers: Vec<Box<dyn EventHandler>> = vec![
            Box::new(DrawCardEventHandler::new()),
            Box::new(StartGameEventHandler::new()),
            Box::new(PlayerStartTurnEventHandler::new()),
        ];
        let player_a = Box::new(player_a);
        let player_b = Box::new(player_b);
        Dispatcher::new(handlers, player_a, player_b)
    };

    // First, dispatch the StartGame event. Both players draw cards to prepare for gameplay.
    let mut game_state = GameState::new(PlayerId::new(), PlayerId::new());
    dispatcher.dispatch(&StartGameEvent::new().into(), &mut game_state);

    // Then, keep getting player input until the game is over.
    while !game_state.is_game_over() {
        player_take_turn(&mut game_state, &dispatcher);
    }
}

fn player_take_turn(game_state: &mut GameState, dispatcher: &Dispatcher) {
    let event = PlayerStartTurnEvent::new(game_state.player_id_a());
    dispatcher.dispatch(&event.into(), game_state);
}

fn init_logger() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format_timestamp_millis();
    builder.init();
}
