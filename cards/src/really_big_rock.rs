use async_trait::async_trait;
use protocol::entities::{BoardPos, Id, Position, RowId, UnitCardInstanceId};
use salt_engine::{
    cards::{actions::UponDeathAction, CardDefinition, UnitCardDefinition},
    game_logic::{events::PosTakesDamageEvent, EventDispatcher},
    game_state::{board::BoardView, GameState},
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

    fn upon_death(&self) -> Box<dyn salt_engine::cards::actions::UponDeathAction> {
        Box::new(DeathAction)
    }
}

struct DeathAction;

#[async_trait]
impl UponDeathAction for DeathAction {
    async fn action(
        &self,
        instance_id: UnitCardInstanceId,
        died_at_pos: BoardPos,
        state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        if died_at_pos.row_id != RowId::FrontRow {
            return;
        }

        let instance = state.board().creature_instance(instance_id);

        let width = instance.width();

        for i in 0..width {
            let index = died_at_pos.row_index + i;
            let behind_pos = BoardPos::new(died_at_pos.player_id, RowId::BackRow, index);
            let event = PosTakesDamageEvent::new(behind_pos, 1);

            dispatcher.dispatch(event, state).await;
        }
    }
}
