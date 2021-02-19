use crate::{
    game_logic::cards::Position,
    game_state::{
        board::{BoardPos, RowId},
        GameState, PlayerId, UnitCardInstanceId,
    },
};

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct SummonCreatureFromHandEvent {
    player_id: PlayerId,
    board_pos: BoardPos,
    hand_card_id: UnitCardInstanceId,
}

impl SummonCreatureFromHandEvent {
    pub fn new(player_id: PlayerId, board_pos: BoardPos, hand_card_id: UnitCardInstanceId) -> Self {
        Self {
            player_id,
            board_pos,
            hand_card_id,
        }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn board_pos(&self) -> BoardPos {
        self.board_pos
    }

    pub fn hand_card_id(&self) -> UnitCardInstanceId {
        self.hand_card_id
    }
}

impl Event for SummonCreatureFromHandEvent {
    fn validate(&self, game_state: &GameState) -> super::Result {
        validate_is_players_side(self, game_state)?;
        validate_respects_placeableat(self, game_state)?;
        validate_player_has_enough_mana(self, game_state)?;

        Ok(())
    }
}

impl Into<GameEvent> for SummonCreatureFromHandEvent {
    fn into(self) -> GameEvent {
        GameEvent::SummonCreatureFromHand(self)
    }
}

fn validate_is_players_side(
    event: &SummonCreatureFromHandEvent,
    _game_state: &GameState,
) -> super::Result {
    let player_id = event.player_id();
    let requested_pos = event.board_pos();

    if requested_pos.player_id != player_id {
        Err(format!(
            "Player {:?} cannot summon a creature on player {:?}'s side",
            player_id, requested_pos.player_id
        )
        .into())
    } else {
        Ok(())
    }
}

fn validate_player_has_enough_mana(
    event: &SummonCreatureFromHandEvent,
    game_state: &GameState,
) -> super::Result {
    let card = game_state
        .hand(event.player_id())
        .card(event.hand_card_id());
    let mana_cost = card.definition().cost() as u32;
    let player_mana = game_state.player_mana(event.player_id());

    if player_mana >= mana_cost {
        Ok(())
    } else {
        Err(format!(
            "Card costs {} mana, but player only has {}.",
            mana_cost, player_mana
        )
        .into())
    }
}

fn validate_respects_placeableat(
    event: &SummonCreatureFromHandEvent,
    game_state: &GameState,
) -> super::Result {
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
        Err(format!(
            "Cannot place in {:?} when card is only placeable at {:?}",
            attempted_row, placeable_at
        )
        .into())
    } else {
        Ok(())
    }
}
