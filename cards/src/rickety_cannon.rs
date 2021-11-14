use async_trait::async_trait;
use log::info;
use salt_engine::{
    cards::{
        actions::{UponSummonAction, UponTurnStartAction},
        CardDefinition, Position, UnitCardDefinition,
    },
    game_logic::{events::PosTakesDamageEvent, EventDispatcher},
    game_state::{
        board::{BoardPos, BoardView},
        GameState, InstanceState, MakePlayerView, UnitCardInstanceId,
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

    fn upon_summon(&self) -> Box<dyn UponSummonAction> {
        Box::new(SummonAction)
    }

    fn upon_turn_start(&self) -> Box<dyn UponTurnStartAction> {
        Box::new(TurnStartAction)
    }
}

struct TurnStartAction;

#[async_trait]
impl UponTurnStartAction for TurnStartAction {
    async fn action(
        &self,
        instance_id: UnitCardInstanceId,
        state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let instance = state.board().creature_instance(instance_id);
        let cannon_target = instance.state();

        if let Some(InstanceState::Pos(pos)) = cannon_target {
            info!("Rickety Cannon fires a shot at {:?}", pos);
            let damage_event = PosTakesDamageEvent::new(pos, 1);
            dispatcher.dispatch(damage_event, state).await;
        }
    }
}

struct SummonAction;

#[async_trait]
impl UponSummonAction for SummonAction {
    async fn action(
        &self,
        _card_id: UnitCardInstanceId,
        summoned_to_pos: BoardPos,
        state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        // let summoner = summoned_to_pos.player_id;

        // let _pos = dispatcher
        //     .player_prompter(summoned_to_pos.player_id)
        //     .prompt_slot(&state.player_view(summoner));

        // todo!();
        // instance.set_state(Some(InstanceState::Pos(pos)));
    }
}
