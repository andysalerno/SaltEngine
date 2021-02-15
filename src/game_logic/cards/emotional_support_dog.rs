use crate::{
    game_logic::{
        buff::{Buff, BuffSourceId},
        BuffInstanceId,
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
        "Frontbuff: +1/+1"
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
struct EmotionalSupportDogBuff {
    instance_id: BuffInstanceId,
    source_id: BuffSourceId,
}

impl EmotionalSupportDogBuff {
    pub fn new(source_id: PassiveEffectInstanceId) -> Self {
        Self {
            instance_id: BuffInstanceId::new(),
            source_id: BuffSourceId::Passive(source_id),
        }
    }
}

impl Buff for EmotionalSupportDogBuff {
    fn attack_amount(&self) -> i32 {
        1
    }

    fn health_amount(&self) -> i32 {
        1
    }

    fn instance_id(&self) -> BuffInstanceId {
        self.instance_id
    }

    fn source_id(&self) -> BuffSourceId {
        self.source_id
    }

    fn definition_id(&self) -> Id {
        todo!()
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
            let doggy_pos = game_state.get_pos_by_id(originator_id);

            if doggy_pos.row_id != RowId::BackRow {
                return;
            }

            let mut front_pos = doggy_pos.clone();
            front_pos.row_id = RowId::FrontRow;

            let front_card = game_state.get_at(front_pos);

            if let Some(front_card) = front_card {
                let id = front_card.id();
                game_state.update_by_id(id, |c| {
                    c.add_buff(Box::new(EmotionalSupportDogBuff::new(instance_id)));
                });
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        game_agent::game_agent::Prompter,
        game_logic::{cards::ReallyBigRock, CreatureSetEvent, EventDispatcher},
        game_state::{
            board::{BoardPos, RowId},
            Deck, GameState, PlayerId, UnitCardInstance,
        },
    };

    use super::EmotionalSupportDog;

    #[test]
    fn when_summoned_expects_provides_buff() {
        let mut state = make_test_state();
        let mut dispatcher = EventDispatcher::new(Box::new(StubPrompter), Box::new(StubPrompter));

        // Summon the thing that will get buffed.
        let rock = UnitCardInstance::new(Box::new(ReallyBigRock));
        let attack_start = rock.attack();
        let health_start = rock.health();
        let buffed_id = rock.id();
        {
            let summon_at = BoardPos::new(state.player_a_id(), RowId::FrontRow, 3);
            let summon_doggy_event = CreatureSetEvent::new(state.player_a_id(), rock, summon_at);
            dispatcher.dispatch(summon_doggy_event, &mut state);
        }

        {
            let doggy = UnitCardInstance::new(Box::new(EmotionalSupportDog));
            let summon_at = BoardPos::new(state.player_a_id(), RowId::BackRow, 3);
            let summon_doggy_event = CreatureSetEvent::new(state.player_a_id(), doggy, summon_at);
            dispatcher.dispatch(summon_doggy_event, &mut state);
        }

        let rock_updated_attack = state.get_by_id(buffed_id).attack();
        let rock_updated_health = state.get_by_id(buffed_id).health();

        assert_eq!(attack_start + 1, rock_updated_attack);
        assert_eq!(health_start + 1, rock_updated_health);
    }

    fn make_test_state() -> GameState {
        let player_a_deck = Deck::new(Vec::new());
        let player_b_deck = Deck::new(Vec::new());

        GameState::initial_state(
            PlayerId::new(),
            player_a_deck,
            PlayerId::new(),
            player_b_deck,
        )
    }

    #[derive(Debug)]
    struct StubPrompter;

    impl Prompter for StubPrompter {
        fn prompt_slot(&self, game_state: &GameState) -> crate::game_state::board::BoardPos {
            todo!()
        }

        fn prompt_player_slot(&self, game_state: &GameState) -> crate::game_state::board::BoardPos {
            todo!()
        }

        fn prompt_opponent_slot(
            &self,
            game_state: &GameState,
        ) -> crate::game_state::board::BoardPos {
            todo!()
        }

        fn prompt_creature_pos(
            &self,
            game_state: &GameState,
        ) -> crate::game_state::board::BoardPos {
            todo!()
        }

        fn prompt_player_creature_pos(
            &self,
            game_state: &GameState,
        ) -> crate::game_state::board::BoardPos {
            todo!()
        }

        fn prompt_opponent_creature_pos(
            &self,
            game_state: &GameState,
        ) -> crate::game_state::board::BoardPos {
            todo!()
        }
    }
}
