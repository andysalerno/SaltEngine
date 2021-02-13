use crate::{
    game_logic::{CreatureTakesDamageEvent, GameEvent, PosTakesDamageEvent},
    game_state::{
        board::{BoardPos, RowId},
        GameState, UnitCardInstanceId,
    },
    id::Id,
};

use super::{CardDefinition, Position, UnitCardDefinition};

#[derive(Debug, Clone)]
pub struct ReallyBigRock;

impl ReallyBigRock {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for ReallyBigRock {
    fn title(&self) -> &str {
        "Really Big Rock"
    }

    fn cost(&self) -> i32 {
        3
    }

    fn flavor_text(&self) -> &str {
        "Not just a boulder."
    }

    fn text(&self) -> &str {
        "Defender\nOn death, deal\n1 damage to slots\n behind this."
    }
}

impl UnitCardDefinition for ReallyBigRock {
    fn attack(&self) -> i32 {
        0
    }

    fn health(&self) -> i32 {
        //4
        1
    }

    fn row_width(&self) -> usize {
        2
    }

    fn is_defender(&self) -> bool {
        true
    }

    fn placeable_at(&self) -> Position {
        Position::Either
    }

    fn upon_death(&self, own_id: UnitCardInstanceId, game_state: &GameState) -> Vec<GameEvent> {
        let my_pos = game_state.get_pos_by_id(own_id);

        if my_pos.row_id != RowId::FrontRow {
            return Vec::new();
        }

        let width = game_state.get_by_id(own_id).width();

        let mut events = Vec::new();

        for i in 0..width {
            let index = my_pos.row_index + i;
            let behind_pos = BoardPos::new(my_pos.player_id, RowId::BackRow, index);
            let event = PosTakesDamageEvent::new(behind_pos, 1);
            events.push(event.into());
        }

        events
    }
}
