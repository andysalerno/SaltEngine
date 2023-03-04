use cards::SleepingDog;
use engine::{
    event::EventHandler, Card, CardId, ClientChannel, Dispatcher, FromClient, FromServer, GamePos,
    GameState, PlayerId,
};
use events::{
    CreatureAttacksTargetEvent, CreatureAttacksTargetEventHandler, CreatureTakesDamageEventHandler,
    DrawCardEventHandler, GainManaEventHandler, PlayerEndTurnEvent, PlayerEndTurnEventHandler,
    PlayerStartTurnEvent, PlayerStartTurnEventHandler, PlayerSummonsCreatureEvent,
    PlayerSummonsCreatureEventHandler, StartGameEvent, StartGameEventHandler,
};
use log::info;

mod websocket_player;

fn main() {
    init_logger();

    let player_a_id = PlayerId::new();
    let player_b_id = PlayerId::new();

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
            Box::new(GainManaEventHandler::new()),
            Box::new(PlayerSummonsCreatureEventHandler::new()),
            Box::new(CreatureAttacksTargetEventHandler::new()),
            Box::new(CreatureTakesDamageEventHandler::new()),
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

    // Loop while player takes actions - break when they end the turn or quit.
    loop {
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

                return;
            }
            engine::FromClient::SummonFromHand {
                card_id,
                target_pos,
            } => {
                let event = PlayerSummonsCreatureEvent::new(player_turn, card_id, target_pos);

                dispatcher.dispatch(event, game_state);
            }
            FromClient::Attack {
                attacker_card_id,
                target_card_id,
            } => {
                let event =
                    CreatureAttacksTargetEvent::new(player_turn, attacker_card_id, target_card_id);

                dispatcher.dispatch(event, game_state);
            }
        }
    }
}

fn init_logger() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format_timestamp_millis();
    builder.init();
}
