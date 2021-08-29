use async_trait::async_trait;
use log::info;
use salt_engine::{
    cards::{actions::UponTurnEndAction, CardDefinition, Position, UnitCardDefinition},
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

    fn upon_turn_end(&self) -> Box<dyn UponTurnEndAction> {
        Box::new(TurnEndAction)
    }
}

struct TurnEndAction;

#[async_trait]
impl UponTurnEndAction for TurnEndAction {
    async fn action(
        &self,
        instance_id: UnitCardInstanceId,
        state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let instance_pos = state.board().pos_with_creature(instance_id);

        if let Some(companion) = state.board().companion_creature(instance_pos) {
            // Heal the target for 2
            let heal_amount = 2;
            let heal_event = CreatureHealedEvent::new(companion.id(), heal_amount);

            {
                let target_creature = state.board().creature_instance(companion.id());
                info!(
                    "Priest of the Lowland heals companion {} for 2",
                    target_creature.definition().title()
                );
            }

            dispatcher.dispatch(heal_event, state).await;
        }
    }
}
