use crate::{
    game_logic::{EventDispatcher, PosTakesDamageEvent},
    game_state::{
        board::{BoardPos, RowId},
        GameState, InstanceState, UnitCardInstance,
    },
    id::Id,
};

use super::{CardDefinition, Position, UnitCardDefinition};

#[derive(Debug, Clone)]
pub struct RicketyCannon;

impl RicketyCannon {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for RicketyCannon {
    fn title(&self) -> &str {
        "Rickety cannon"
    }

    fn cost(&self) -> i32 {
        2
    }

    fn flavor_text(&self) -> &str {
        "yep"
    }

    fn text(&self) -> &str {
        "Back\nSummon: pick a slot.\nAt the start of\nyour turn, deal\n1 damage there."
    }
}

impl UnitCardDefinition for RicketyCannon {
    fn attack(&self) -> i32 {
        0
    }

    fn health(&self) -> i32 {
        2
    }

    fn row_width(&self) -> usize {
        3
    }

    fn placeable_at(&self) -> Position {
        Position::Back
    }

    fn upon_summon(
        &self,
    ) -> Box<dyn FnOnce(&mut UnitCardInstance, BoardPos, &mut GameState, &mut EventDispatcher)>
    {
        Box::new(|instance, _summoned_to_pos, game_state, dispatcher| {
            let pos = dispatcher.player_prompter().prompt_slot(game_state);

            instance.set_state(Some(InstanceState::Pos(pos)));

            println!("Hello from callback :)");

            let behind_pos = BoardPos::new(game_state.cur_player_id(), RowId::BackRow, 0);
            let event = PosTakesDamageEvent::new(behind_pos, 1);

            dispatcher.dispatch(event, game_state);
        })
    }
}
