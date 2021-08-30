use super::{EventDispatcher, PassiveEffectDefinition};
use crate::game_state::{
    board::BoardPos, GameState, MakePlayerView, PlayerId, UnitCardInstance, UnitCardInstanceId,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Describes which board positions
/// this creature card may occupy.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Position {
    /// The front side of the board.
    Front,

    /// The back side of the board.
    Back,

    /// Either the front or the back sides of the board..
    Either,
}

/// The most general definition that cards of all types must implement.
pub trait CardDefinition: std::fmt::Debug + Send + Sync {
    fn title(&self) -> &str;
    fn cost(&self) -> i32;
    fn text(&self) -> &str;
    fn flavor_text(&self) -> &str;
}

/// A `Card` that can be placed as a unit on the board.
pub trait UnitCardDefinition: CardDefinition {
    fn attack(&self) -> i32;
    fn health(&self) -> i32;
    fn row_width(&self) -> usize;
    fn placeable_at(&self) -> Position;

    /// The card may provide logic that is executed when it is summoned from a the player's hand.
    /// The boxed function is provided the instance of the card being summoned,
    /// the current game state of the board as it was summoned,
    /// and the event dispatcher, in case the card's summoning effect requries dispatching more events.
    fn upon_summon(&self) -> Box<dyn actions::UponSummonAction> {
        Box::new(actions::DoNothingAction)
    }

    fn upon_death(&self) -> Box<dyn actions::UponDeathAction> {
        Box::new(actions::DoNothingAction)
    }

    fn upon_receive_damage(&self) -> Box<dyn actions::UponReceiveDamageAction> {
        Box::new(actions::DoNothingAction)
    }

    fn upon_turn_start(&self) -> Box<dyn actions::UponTurnStartAction> {
        Box::new(actions::DoNothingAction)
    }

    fn upon_turn_end(&self) -> Box<dyn actions::UponTurnEndAction> {
        Box::new(actions::DoNothingAction)
    }

    fn passive_effect(&self) -> Option<Box<dyn PassiveEffectDefinition>> {
        None
    }

    // TODO: or naming "guardian"?
    fn is_defender(&self) -> bool {
        false
    }

    fn is_hidden(&self) -> bool {
        false
    }

    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    fn make_instance(self) -> UnitCardInstance
    where
        Self: Sized + 'static,
    {
        let boxed: Box<dyn UnitCardDefinition> = self.boxed();
        UnitCardInstance::new(boxed)
    }
}

pub trait UnitCardDefinitionView {
    fn title(&self) -> &str;
    fn cost(&self) -> i32;
    fn text(&self) -> &str;
    fn flavor_text(&self) -> &str;
    fn attack(&self) -> i32;
    fn health(&self) -> i32;
    fn row_width(&self) -> usize;
    fn placeable_at(&self) -> Position;
}

impl UnitCardDefinitionView for dyn UnitCardDefinition {
    fn title(&self) -> &str {
        self.title()
    }

    fn cost(&self) -> i32 {
        self.cost()
    }

    fn text(&self) -> &str {
        self.text()
    }

    fn flavor_text(&self) -> &str {
        self.flavor_text()
    }

    fn attack(&self) -> i32 {
        self.attack()
    }

    fn health(&self) -> i32 {
        self.health()
    }

    fn row_width(&self) -> usize {
        self.row_width()
    }

    fn placeable_at(&self) -> Position {
        self.placeable_at()
    }
}

pub mod player_view {
    use super::*;

    #[derive(Debug, Serialize, Clone, Deserialize)]
    pub struct UnitCardDefinitionPlayerView {
        title: String,
        cost: i32,
        text: String,
        flavor_text: String,
        attack: i32,
        health: i32,
        row_width: usize,
        placeable_at: Position,
    }

    // impl MakePlayerView for Box<dyn UnitCardDefinition> {
    //     type TOut = UnitCardDefinitionPlayerView;

    //     fn player_view(&self, _player_viewing: PlayerId) -> UnitCardDefinitionPlayerView {
    //         UnitCardDefinitionPlayerView {
    //             title: self.title().to_string(),
    //             cost: self.cost(),
    //             text: self.text().to_string(),
    //             flavor_text: self.flavor_text().to_string(),
    //             attack: self.attack(),
    //             health: self.health(),
    //             row_width: self.row_width(),
    //             placeable_at: self.placeable_at(),
    //         }
    //     }
    // }

    impl<'a> MakePlayerView<'a> for dyn UnitCardDefinition {
        type TOut = UnitCardDefinitionPlayerView;

        fn player_view(&'a self, _player_viewing: PlayerId) -> UnitCardDefinitionPlayerView {
            UnitCardDefinitionPlayerView {
                title: self.title().to_string(),
                cost: self.cost(),
                text: self.text().to_string(),
                flavor_text: self.flavor_text().to_string(),
                attack: self.attack(),
                health: self.health(),
                row_width: self.row_width(),
                placeable_at: self.placeable_at(),
            }
        }
    }

    impl UnitCardDefinitionView for UnitCardDefinitionPlayerView {
        fn title(&self) -> &str {
            self.title.as_str()
        }

        fn cost(&self) -> i32 {
            self.cost
        }

        fn text(&self) -> &str {
            self.text.as_str()
        }

        fn flavor_text(&self) -> &str {
            self.flavor_text.as_str()
        }

        fn attack(&self) -> i32 {
            self.attack
        }

        fn health(&self) -> i32 {
            self.health
        }

        fn row_width(&self) -> usize {
            self.row_width
        }

        fn placeable_at(&self) -> Position {
            self.placeable_at
        }
    }
}

pub mod actions {
    use super::*;

    #[async_trait]
    pub trait UponSummonAction: Send + Sync {
        async fn action(
            &self,
            instance: &mut UnitCardInstance,
            pos: BoardPos,
            state: &mut GameState,
            dispatcher: &mut EventDispatcher,
        );
    }

    #[async_trait]
    pub trait UponDeathAction: Send + Sync {
        async fn action(
            &self,
            instance: &mut UnitCardInstance,
            pos: BoardPos,
            state: &mut GameState,
            dispatcher: &mut EventDispatcher,
        );
    }

    #[async_trait]
    pub trait UponReceiveDamageAction: Send + Sync {
        async fn action(
            &self,
            instance_id: UnitCardInstanceId,
            state: &mut GameState,
            dispatcher: &mut EventDispatcher,
        );
    }

    #[async_trait]
    pub trait UponTurnStartAction: Send + Sync {
        async fn action(
            &self,
            instance_id: UnitCardInstanceId,
            state: &mut GameState,
            dispatcher: &mut EventDispatcher,
        );
    }

    #[async_trait]
    pub trait UponTurnEndAction: Send + Sync {
        async fn action(
            &self,
            instance_id: UnitCardInstanceId,
            state: &mut GameState,
            dispatcher: &mut EventDispatcher,
        );
    }

    pub(super) struct DoNothingAction;

    #[async_trait]
    impl UponSummonAction for DoNothingAction {
        async fn action(
            &self,
            _instance: &mut UnitCardInstance,
            _pos: BoardPos,
            _state: &mut GameState,
            _dispatcher: &mut EventDispatcher,
        ) {
            // Do nothing by default
        }
    }

    #[async_trait]
    impl UponDeathAction for DoNothingAction {
        async fn action(
            &self,
            _instance: &mut UnitCardInstance,
            _pos: BoardPos,
            _state: &mut GameState,
            _dispatcher: &mut EventDispatcher,
        ) {
            // Do nothing by default
        }
    }

    #[async_trait]
    impl UponReceiveDamageAction for DoNothingAction {
        async fn action(
            &self,
            _instance_id: UnitCardInstanceId,
            _state: &mut GameState,
            _dispatcher: &mut EventDispatcher,
        ) {
            // Do nothing by default
        }
    }

    #[async_trait]
    impl UponTurnStartAction for DoNothingAction {
        async fn action(
            &self,
            _instance_id: UnitCardInstanceId,
            _state: &mut GameState,
            _dispatcher: &mut EventDispatcher,
        ) {
            // Do nothing by default
        }
    }

    #[async_trait]
    impl UponTurnEndAction for DoNothingAction {
        async fn action(
            &self,
            _instance_id: UnitCardInstanceId,
            _state: &mut GameState,
            _dispatcher: &mut EventDispatcher,
        ) {
            // Do nothing by default
        }
    }
}
