use crate::{
    game_logic::{
        cards::Position, event_handlers::EventHandler, CreatureSetEvent, EventDispatcher,
        PlayerSpendManaEvent, SummonCreatureFromHandEvent,
    },
    game_state::{board::RowId, GameState},
};

#[derive(Default)]
pub struct SummonCreatureFromHandEventHandler;

impl EventHandler for SummonCreatureFromHandEventHandler {
    type Event = SummonCreatureFromHandEvent;

    fn handle(
        &self,
        event: SummonCreatureFromHandEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        validate(&event, game_state);

        let player_id = event.player_id();
        let hand_card_id = event.hand_card_id();

        let card_from_hand = game_state.hand_mut(player_id).take_card(hand_card_id);

        let creature_name = card_from_hand.definition().title();
        let mana_amount = card_from_hand.definition().cost();
        println!("Player {:?} summons {}", player_id, creature_name);

        dispatcher.dispatch(
            PlayerSpendManaEvent::new(player_id, mana_amount as u32),
            game_state,
        );

        let upon_summon = card_from_hand.definition().upon_summonz();

        // Execute the "upon summon" behavior
        (upon_summon)(hand_card_id, game_state, dispatcher);

        let pos = event.board_pos();

        dispatcher.dispatch(
            CreatureSetEvent::new(player_id, card_from_hand, pos),
            game_state,
        );
    }
}

fn validate(event: &SummonCreatureFromHandEvent, game_state: &GameState) {
    validate_respects_placeableat(event, game_state);
    validate_is_players_side(event, game_state);
}

fn validate_is_players_side(event: &SummonCreatureFromHandEvent, _game_state: &GameState) {
    let player_id = event.player_id();
    let requested_pos = event.board_pos();

    if requested_pos.player_id != player_id {
        panic!(
            "Player {:?} cannot summon a creature on player {:?}'s side",
            player_id, requested_pos.player_id
        );
    }
}

fn validate_respects_placeableat(event: &SummonCreatureFromHandEvent, game_state: &GameState) {
    let player_id = event.player_id();
    let card_id = event.hand_card_id();

    let placeable_at = game_state
        .hand(player_id)
        .card(card_id)
        .definition()
        .placeable_at();

    let attempted_row = event.board_pos().row_id;

    if (placeable_at == Position::Back && attempted_row == RowId::FrontRow)
        || (placeable_at == Position::Front && attempted_row == RowId::BackRow)
    {
        panic!(
            "Cannot place in {:?} when card is only placeable at {:?}",
            attempted_row, placeable_at
        );
    }
}
