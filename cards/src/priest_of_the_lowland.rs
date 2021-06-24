use log::info;
use salt_engine::{
    cards::{CardDefinition, Position, UnitCardDefinition},
    game_logic::{CreatureHealedEvent, EventDispatcher},
    game_state::{board::BoardView, GameState, UnitCardInstanceId},
    id::Id,
};

#[derive(Debug, Clone)]
pub struct PriestOfTheLowland;

impl PriestOfTheLowland {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for PriestOfTheLowland {
    fn title(&self) -> &str {
        "Priest of the Lowland"
    }

    fn cost(&self) -> i32 {
        1
    }

    fn flavor_text(&self) -> &str {
        "todo"
    }

    fn text(&self) -> &str {
        "Back
Heal the Companion
for 2 health at
the end of your turn."
    }
}

impl UnitCardDefinition for PriestOfTheLowland {
    fn attack(&self) -> i32 {
        0
    }

    fn health(&self) -> i32 {
        2
    }

    fn row_width(&self) -> usize {
        1
    }

    fn placeable_at(&self) -> Position {
        Position::Back
    }

    fn upon_turn_end(
        &self,
    ) -> Box<dyn FnOnce(UnitCardInstanceId, &mut GameState, &mut EventDispatcher)> {
        Box::new(|id, game_state, dispatcher| {
            let instance_pos = game_state.board().pos_with_creature(id);

            if let Some(companion) = game_state.board().companion_creature(instance_pos) {
                // Heal the target for 2
                let heal_amount = 2;
                let heal_event = CreatureHealedEvent::new(companion.id(), heal_amount);

                {
                    let target_creature = game_state.board().creature_instance(companion.id());
                    info!(
                        "Priest of the Lowland heals companion {} for 2",
                        target_creature.definition().title()
                    );
                }

                dispatcher.dispatch(heal_event, game_state);
            }
        })
    }
}
