use salt_engine::{
    cards::{actions::UponEventAction, CardDefinition, Position, UnitCardDefinition},
    game_logic::{events::GameEvent, EventDispatcher},
    game_state::GameState,
    id::Id,
};

#[derive(Debug, Clone)]
pub struct GrandmaTheSoother;

impl GrandmaTheSoother {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for GrandmaTheSoother {
    fn title(&self) -> &str {
        "Grandma The Soother"
    }

    fn cost(&self) -> i32 {
        2
    }

    fn flavor_text(&self) -> &str {
        "todo"
    }

    fn text(&self) -> &str {
        "Back.
When her companion
takes damage,
draw a card."
    }
}

impl UnitCardDefinition for GrandmaTheSoother {
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

    fn pre_event_action(
        &self,
        _event: GameEvent,
        _game_state: &GameState,
        _dispatcher: &mut EventDispatcher,
    ) -> Option<Box<dyn UponEventAction>> {
        None
    }
}
