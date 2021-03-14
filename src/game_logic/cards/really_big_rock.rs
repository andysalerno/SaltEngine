use super::{CardDefinition, Position, UnitCardDefinition};
use crate::{
    game_logic::PosTakesDamageEvent,
    game_state::{
        board::{BoardPos, RowId},
        GameState,
    },
    id::Id,
};

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

    fn upon_death(
        &self,
    ) -> Box<
        dyn FnOnce(
            &mut crate::game_state::UnitCardInstance,
            BoardPos,
            &mut GameState,
            &mut crate::game_logic::EventDispatcher,
        ),
    > {
        Box::new(|instance, died_at_pos, game_state, dispatcher| {
            if died_at_pos.row_id != RowId::FrontRow {
                return;
            }

            let width = instance.width();

            for i in 0..width {
                let index = died_at_pos.row_index + i;
                let behind_pos = BoardPos::new(died_at_pos.player_id, RowId::BackRow, index);
                let event = PosTakesDamageEvent::new(behind_pos, 1);

                dispatcher.dispatch(event, game_state);
            }
        })
    }
}
