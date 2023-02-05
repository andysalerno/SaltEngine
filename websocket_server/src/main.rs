use cards::SleepingDog;
use engine::{
    event::EventHandler, Card, ClientChannel, Dispatcher, FromClient, FromServer, GameState,
    PlayerId,
};
use events::{
    DrawCardEventHandler, PlayerEndTurnEvent, PlayerEndTurnEventHandler, PlayerStartTurnEvent,
    PlayerStartTurnEventHandler, StartGameEvent, StartGameEventHandler,
};
use log::info;
use serde::Serialize;

mod websocket_player;

fn main() {
    init_logger();

    let player_a_id = PlayerId::new();
    let player_b_id = PlayerId::new();

    let message = FromClient::EndTurn;
    let text = serde_json::to_string(&message).unwrap();

    info!("EndTurn message: {text:?}");

    let (player_a, player_b) = websocket_player::accept_connections();

    let player_a_channel = ClientChannel::new(player_a_id, Box::new(player_a));
    let player_b_channel = ClientChannel::new(player_b_id, Box::new(player_b));

    player_a_channel.send(FromServer::Hello(player_a_id, player_b_id));
    player_b_channel.send(FromServer::Hello(player_b_id, player_a_id));

    info!("Both players connected. Starting game.");
    let dispatcher = {
        let handlers: Vec<Box<dyn EventHandler>> = vec![
            Box::new(DrawCardEventHandler::new()),
            Box::new(StartGameEventHandler::new()),
            Box::new(PlayerStartTurnEventHandler::new()),
            Box::new(PlayerEndTurnEventHandler::new()),
        ];
        Dispatcher::new(handlers, player_a_channel, player_b_channel)
    };

    // First, dispatch the StartGame event. Both players draw cards to prepare for gameplay.
    let mut game_state = GameState::new(player_a_id, player_b_id);

    // Add cards to decks for both players.
    for player_id in [player_a_id, player_b_id] {
        let player_a_deck = game_state.deck_mut(player_id);
        player_a_deck.add_card_to_bottom(Card::new(SleepingDog::make_definition()));
        player_a_deck.add_card_to_bottom(Card::new(SleepingDog::make_definition()));
        player_a_deck.add_card_to_bottom(Card::new(SleepingDog::make_definition()));
        player_a_deck.add_card_to_bottom(Card::new(SleepingDog::make_definition()));
        player_a_deck.add_card_to_bottom(Card::new(SleepingDog::make_definition()));
        player_a_deck.add_card_to_bottom(Card::new(SleepingDog::make_definition()));
    }

    dispatcher.dispatch(StartGameEvent::new(), &mut game_state);

    // Then, keep getting player input until the game is over.
    while !game_state.is_game_over() {
        player_take_turn(&mut game_state, &dispatcher);
    }
}

fn player_take_turn(game_state: &mut GameState, dispatcher: &Dispatcher) {
    let player_turn = game_state.cur_player_turn();
    let next_base_mana = game_state.player_base_mana(player_turn) + 1;
    let event = PlayerStartTurnEvent::new(player_turn, next_base_mana);
    dispatcher.dispatch(event, game_state);

    let message = if player_turn == game_state.player_id_a() {
        dispatcher.player_a_channel().try_receive()
    } else if player_turn == game_state.player_id_b() {
        dispatcher.player_b_channel().try_receive()
    } else {
        panic!("Unknown player id.")
    };

    info!("Received message: {message:?}");

    let message = message.unwrap();

    match message {
        engine::FromClient::EndTurn => {
            let event = PlayerEndTurnEvent::new(player_turn);
            dispatcher.dispatch(event, game_state);
        }
    }
}

fn init_logger() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format_timestamp_millis();
    builder.init();
}
