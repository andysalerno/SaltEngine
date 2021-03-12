use crate::{
    game_logic::{
        buff::{Buff, BuffSourceId},
        BuffBuilder, BuffInstanceId,
    },
    game_state::{board::RowId, UnitCardInstanceId},
    id::Id,
};
use crate::{
    game_logic::{passive_effect::PassiveEffectInstanceId, PassiveEffectDefinition},
    game_state::GameState,
};

use super::{CardDefinition, Position, UnitCardDefinition};

#[derive(Debug, Clone)]
pub struct EmotionalSupportDog;

impl EmotionalSupportDog {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for EmotionalSupportDog {
    fn title(&self) -> &str {
        "Emo Sup Dog"
        //"Emotional Support Dog"
    }

    fn cost(&self) -> i32 {
        2
    }

    fn flavor_text(&self) -> &str {
        "But really, aren't all dogs Emotional Support Dogs?"
    }

    fn text(&self) -> &str {
        "Back
Companion has +1/+1."
    }
}

impl UnitCardDefinition for EmotionalSupportDog {
    fn attack(&self) -> i32 {
        1
    }

    fn health(&self) -> i32 {
        1
    }

    fn row_width(&self) -> usize {
        1
    }

    fn passive_effect(&self) -> Option<Box<dyn PassiveEffectDefinition>> {
        Some(Box::new(EmotionalSupportDogPassiveDefinition::new()))
    }

    fn placeable_at(&self) -> Position {
        Position::Back
    }
}

#[derive(Debug)]
struct EmotionalSupportDogPassiveDefinition {
    definition_id: Id,
}

impl EmotionalSupportDogPassiveDefinition {
    pub fn new() -> Self {
        Self {
            // TODO: replace with constant
            definition_id: Id::new(),
        }
    }
}

impl PassiveEffectDefinition for EmotionalSupportDogPassiveDefinition {
    fn definition_id(&self) -> Id {
        todo!()
    }

    fn update(
        &self,
    ) -> Box<dyn FnOnce(PassiveEffectInstanceId, UnitCardInstanceId, &mut GameState)> {
        Box::new(move |instance_id, originator_id, game_state| {
            let doggy_pos = game_state.board().position_with_creature(originator_id);

            if let Some(companion) = game_state.board().companion_creature(doggy_pos) {
                let id = companion.id();

                let buff = BuffBuilder::new(BuffSourceId::Passive(instance_id), Id::new())
                    .attack(1)
                    .health(1)
                    .build();

                game_state.update_by_id(id, |c| {
                    c.add_buff(Box::new(buff));
                });
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        game_logic::cards::UnitCardDefinition,
        game_logic::*,
        game_logic::{cards::ReallyBigRock, CreatureSetEvent},
        game_state::board::{BoardPos, RowId},
        game_state::make_test_state,
    };

    use super::EmotionalSupportDog;

    #[test]
    fn when_summoned_expects_provides_buff() {
        let mut state = make_test_state();
        let mut dispatcher = make_default_dispatcher();

        // Summon the thing that will get buffed.
        let rock = ReallyBigRock.make_instance();
        let attack_start = rock.attack();
        let health_start = rock.health();
        let buffed_id = rock.id();
        {
            let summon_at = BoardPos::new(state.player_a_id(), RowId::FrontRow, 3);
            let summon_doggy_event = CreatureSetEvent::new(state.player_a_id(), rock, summon_at);
            dispatcher.dispatch(summon_doggy_event, &mut state);
        }

        {
            let doggy = EmotionalSupportDog.make_instance();
            let summon_at = BoardPos::new(state.player_a_id(), RowId::BackRow, 3);
            let summon_doggy_event = CreatureSetEvent::new(state.player_a_id(), doggy, summon_at);
            dispatcher.dispatch(summon_doggy_event, &mut state);
        }

        let rock_updated_attack = state.board().creature_instance(buffed_id).attack();
        let rock_updated_health = state.board().creature_instance(buffed_id).health();

        assert_eq!(attack_start + 1, rock_updated_attack);
        assert_eq!(health_start + 1, rock_updated_health);
    }
}
