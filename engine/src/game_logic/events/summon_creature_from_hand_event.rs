use super::{Event, GameEvent};
use crate::{
    cards::UnitCardDefinitionView,
    game_logic::cards::Position,
    game_state::{
        board::{BoardPos, BoardView, RowId},
        GameStateView, HandView, PlayerId, UnitCardInstanceId, UnitCardInstanceView,
    },
};
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SummonCreatureFromHandEvent {
    player_id: PlayerId,
    board_pos: BoardPos,
    hand_card_id: UnitCardInstanceId,
}

impl SummonCreatureFromHandEvent {
    #[must_use]
    pub fn new(player_id: PlayerId, board_pos: BoardPos, hand_card_id: UnitCardInstanceId) -> Self {
        Self {
            player_id,
            board_pos,
            hand_card_id,
        }
    }

    #[must_use]
    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    #[must_use]
    pub fn board_pos(&self) -> BoardPos {
        self.board_pos
    }

    #[must_use]
    pub fn hand_card_id(&self) -> UnitCardInstanceId {
        self.hand_card_id
    }
}

impl Event for SummonCreatureFromHandEvent {
    fn validate<'a, G>(&self, game_state: &'a G) -> super::Result
    where
        G: GameStateView<'a>,
    {
        validation::validate_is_players_side(self, game_state)?;
        validation::validate_slots_available(self, game_state)?;
        validation::validate_respects_placeableat(self, game_state)?;
        validation::validate_player_has_enough_mana(self, game_state)?;

        Ok(())
    }

    fn maybe_client_event(&self) -> Option<super::ClientEventView> {
        let client_event = SummonCreatureFromHandClientEvent {
            player_id: self.player_id,
            board_pos: self.board_pos,
            hand_card_id: self.hand_card_id,
        };

        Some(super::ClientEventView::SummonCreatureFromHand(client_event))
    }
}

impl From<SummonCreatureFromHandEvent> for GameEvent {
    fn from(val: SummonCreatureFromHandEvent) -> Self {
        GameEvent::SummonCreatureFromHand(val)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SummonCreatureFromHandClientEvent {
    pub player_id: PlayerId,
    pub board_pos: BoardPos,
    pub hand_card_id: UnitCardInstanceId,
}

mod validation {
    use super::{
        debug, BoardView, GameStateView, HandView, Position, RowId, SummonCreatureFromHandEvent,
        UnitCardDefinitionView, UnitCardInstanceView,
    };

    pub fn validate_slots_available<'a>(
        event: &SummonCreatureFromHandEvent,
        game_state: &'a impl GameStateView<'a>,
    ) -> super::super::Result {
        debug!("Validating the slots for the summon are not already occupied.");
        let creature_width = game_state
            .hand(event.player_id())
            .card(event.hand_card_id())
            .width();

        debug!("board pos?");
        let requested_pos = event.board_pos();

        debug!("is range in row?");
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
        debug!("is range in row finished");

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

        Ok(())
    }

    pub fn validate_is_players_side<'a>(
        event: &SummonCreatureFromHandEvent,
        _game_state: &'a impl GameStateView<'a>,
    ) -> super::super::Result {
        debug!("Validating the summon is from the player's own side.");
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

    pub fn validate_player_has_enough_mana<'a, G>(
        event: &SummonCreatureFromHandEvent,
        game_state: &'a G,
    ) -> super::super::Result
    where
        G: GameStateView<'a>,
    {
        debug!("Validating the player has enough mana for the summon.");
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

    pub fn validate_respects_placeableat<'a>(
        event: &SummonCreatureFromHandEvent,
        game_state: &'a impl GameStateView<'a>,
    ) -> super::super::Result {
        debug!("Validating the slots are in the card's placable positions.");
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
}
