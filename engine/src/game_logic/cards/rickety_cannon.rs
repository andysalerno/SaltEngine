use super::{CardDefinition, Position, UnitCardDefinition};
use crate::{
    game_logic::{EventDispatcher, PosTakesDamageEvent},
    game_state::{
        board::{BoardPos, BoardView},
        GameState, InstanceState, MakePlayerView, UnitCardInstance, UnitCardInstanceId,
    },
    id::Id,
};

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
        Box::new(|instance, summoned_to_pos, game_state, dispatcher| {
            let summoner = summoned_to_pos.player_id;

            todo!("Prompting still busted");
            // let pos = dispatcher
            //     .player_prompter()
            //     .prompt_slot(&game_state.player_view(summoner));

            //instance.set_state(Some(InstanceState::Pos(pos)));
        })
    }

    fn upon_turn_start(
        &self,
    ) -> Box<dyn FnOnce(UnitCardInstanceId, &mut GameState, &mut EventDispatcher)> {
        Box::new(|id, game_state, dispatcher| {
            let instance = game_state.board().creature_instance(id);
            let cannon_target = instance.state();

            if let Some(InstanceState::Pos(pos)) = cannon_target {
                println!("Rickety Cannon fires a shot at {:?}", pos);
                let damage_event = PosTakesDamageEvent::new(pos, 1);
                dispatcher.dispatch(damage_event, game_state);
            }
        })
    }
}
