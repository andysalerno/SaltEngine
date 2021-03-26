use super::{Event, GameEvent};
use crate::{
    cards::UnitCardDefinitionView,
    game_logic::cards::Position,
    game_state::{
        board::{BoardPos, BoardView, RowId},
        GameStateView, HandView, PlayerId, UnitCardInstanceId, UnitCardInstanceView,
    },
};

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
    fn validate<'a, G>(&self, game_state: &'a G) -> super::Result
    where
        G: GameStateView<'a>,
    {
        validate_is_players_side(self, game_state)?;
        validate_slots_available(self, game_state)?;
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

fn validate_slots_available<'a>(
    event: &SummonCreatureFromHandEvent,
    game_state: &'a impl GameStateView<'a>,
) -> super::Result {
    let creature_width = game_state
        .hand(event.player_id())
        .card(event.hand_card_id())
        .width();

    let requested_pos = event.board_pos();

    if !game_state
        .board()
        .is_range_in_row(requested_pos, creature_width)
    {
        return Err(format!(
            "Creature has width {} and cannot be summoned at {:?}",
            creature_width, requested_pos
        )
        .into());
    }

    for i in 0..creature_width {
        let mut look_pos = requested_pos;
        look_pos.row_index += i;

        if game_state.board().creature_at_pos(look_pos).is_some() {
            return Err(format!(
                "Cannot summon at pos {:?} with width {} since a creature occupies pos {:?}",
                requested_pos, creature_width, look_pos
            )
            .into());
        }
    }

    return Ok(());
}

fn validate_is_players_side<'a>(
    event: &SummonCreatureFromHandEvent,
    _game_state: &'a impl GameStateView<'a>,
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

fn validate_player_has_enough_mana<'a, G>(
    event: &SummonCreatureFromHandEvent,
    game_state: &'a G,
) -> super::Result
where
    G: GameStateView<'a>,
{
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

fn validate_respects_placeableat<'a>(
    event: &SummonCreatureFromHandEvent,
    game_state: &'a impl GameStateView<'a>,
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
