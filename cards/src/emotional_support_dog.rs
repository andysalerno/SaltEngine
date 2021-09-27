use salt_engine::{
    cards::{CardDefinition, Position, UnitCardDefinition},
    game_logic::{
        BuffBuilder, PassiveCompanionBuff, PassiveEffectDefinition, PassiveEffectInstanceId,
    },
    id::Id,
};

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
        let buff = BuffBuilder::new(PassiveEffectInstanceId::new(), Id::new())
            .attack(1)
            .health(1)
            .build();

        let passive = PassiveCompanionBuff::new(Id::new(), Box::new(buff));
        Some(Box::new(passive))
    }

    fn placeable_at(&self) -> Position {
        Position::Back
    }
}

#[cfg(test)]
mod tests {
    use salt_engine::{
        cards::UnitCardDefinition,
        game_logic::events::CreatureSetEvent,
        game_state::board::{BoardPos, BoardView, RowId},
    };

    use crate::{
        tests::{make_dispatcher, make_test_state},
        ReallyBigRock,
    };

    use super::EmotionalSupportDog;

    #[test]
    fn when_summoned_expects_provides_buff() {
        let mut state = make_test_state();
        let mut dispatcher = make_dispatcher(state.player_a_id(), state.player_b_id());

        // Summon the thing that will get buffed.
        let rock = ReallyBigRock.make_instance();
        let attack_start = rock.attack();
        let health_start = rock.health();
        let buffed_id = rock.id();
        {
            state.board_mut().set_pre_summon(rock);
            let summon_at = BoardPos::new(state.player_a_id(), RowId::FrontRow, 3);
            let summon_doggy_event =
                CreatureSetEvent::new(state.player_a_id(), buffed_id, summon_at);
            smol::block_on(async {
                dispatcher.dispatch(summon_doggy_event, &mut state).await;
            });
        }

        {
            let doggy = EmotionalSupportDog.make_instance();
            let doggy_id = doggy.id();
            state.board_mut().set_pre_summon(doggy);
            let summon_at = BoardPos::new(state.player_a_id(), RowId::BackRow, 3);
            let summon_doggy_event =
                CreatureSetEvent::new(state.player_a_id(), doggy_id, summon_at);

            smol::block_on(async {
                dispatcher.dispatch(summon_doggy_event, &mut state).await;
            });
        }

        let rock_updated_attack = state.board().creature_instance(buffed_id).attack();
        let rock_updated_health = state.board().creature_instance(buffed_id).health();

        assert_eq!(attack_start + 1, rock_updated_attack);
        assert_eq!(health_start + 1, rock_updated_health);
    }
}
