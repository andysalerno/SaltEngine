use super::{events::GameEvent, EventDispatcher, PassiveEffectDefinition};
use crate::game_state::{GameState, MakePlayerView, UnitCardInstance};
use async_trait::async_trait;
use protocol::entities::{CreatureInstanceId, Position};
use serde::{Deserialize, Serialize};

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

    fn upon_companion_damaged(&self) -> Box<dyn actions::UponTurnEndAction> {
        Box::new(actions::DoNothingAction)
    }

    fn passive_effect(&self) -> Option<Box<dyn PassiveEffectDefinition>> {
        None
    }

    /// Invoked on each instance before every event is executed.
    fn pre_event_action(
        &self,
        _card_instance_id: CreatureInstanceId,
        _event: &GameEvent,
        _game_state: &GameState,
        _dispatcher: &mut EventDispatcher,
    ) -> Option<Box<dyn actions::UponEventAction>> {
        None
    }

    /// Invoked on each instance after every event is executed.
    fn post_event_action(
        &self,
        _card_instance_id: CreatureInstanceId,
        _event: &GameEvent,
        _game_state: &GameState,
        _dispatcher: &mut EventDispatcher,
    ) -> Option<Box<dyn actions::UponEventAction>> {
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
    use protocol::entities::PlayerId;

    use super::{
        Deserialize, MakePlayerView, Position, Serialize, UnitCardDefinition,
        UnitCardDefinitionView,
    };

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
    use protocol::entities::BoardPos;

    use super::{async_trait, CreatureInstanceId, EventDispatcher, GameState};
    use crate::game_logic::events::GameEvent;

    #[async_trait]
    pub trait UponSummonAction: Send + Sync {
        async fn action(
            &self,
            instance_id: CreatureInstanceId,
            pos: BoardPos,
            state: &mut GameState,
            dispatcher: &mut EventDispatcher,
        );
    }

    #[async_trait]
    pub trait UponDeathAction: Send + Sync {
        async fn action(
            &self,
            instance_id: CreatureInstanceId,
            pos: BoardPos,
            state: &mut GameState,
            dispatcher: &mut EventDispatcher,
        );
    }

    #[async_trait]
    pub trait UponReceiveDamageAction: Send + Sync {
        async fn action(
            &self,
            instance_id: CreatureInstanceId,
            state: &mut GameState,
            dispatcher: &mut EventDispatcher,
        );
    }

    #[async_trait]
    pub trait UponTurnStartAction: Send + Sync {
        async fn action(
            &self,
            instance_id: CreatureInstanceId,
            state: &mut GameState,
            dispatcher: &mut EventDispatcher,
        );
    }

    #[async_trait]
    pub trait UponTurnEndAction: Send + Sync {
        async fn action(
            &self,
            instance_id: CreatureInstanceId,
            state: &mut GameState,
            dispatcher: &mut EventDispatcher,
        );
    }

    #[async_trait]
    pub trait UponCompanionDamagedAction: Send + Sync {
        async fn action(
            &self,
            instance_id: CreatureInstanceId,
            state: &mut GameState,
            dispatcher: &mut EventDispatcher,
        );
    }

    #[async_trait]
    pub trait UponEventAction: Send + Sync {
        async fn action(
            &self,
            event: &GameEvent,
            state: &mut GameState,
            dispatcher: &mut EventDispatcher,
        );
    }

    pub(super) struct DoNothingAction;

    #[async_trait]
    impl UponSummonAction for DoNothingAction {
        async fn action(
            &self,
            _instance_id: CreatureInstanceId,
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
            _instance_id: CreatureInstanceId,
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
            _instance_id: CreatureInstanceId,
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
            _instance_id: CreatureInstanceId,
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
            _instance_id: CreatureInstanceId,
            _state: &mut GameState,
            _dispatcher: &mut EventDispatcher,
        ) {
            // Do nothing by default
        }
    }
}

mod builder {
    use protocol::entities::CreatureInstanceId;

    use super::{
        actions::{
            DoNothingAction, UponDeathAction, UponEventAction, UponReceiveDamageAction,
            UponSummonAction, UponTurnEndAction, UponTurnStartAction,
        },
        CardDefinition, Position, UnitCardDefinition,
    };
    use crate::{
        game_logic::{events::GameEvent, EventDispatcher, PassiveEffectDefinition},
        game_state::{GameState, UnitCardInstance},
    };

    /// A builder for unit card definitions.
    pub struct Builder {
        inner: BuiltUnitCardDefinition,
    }

    impl Builder {
        pub fn new() -> Self {
            Self {
                inner: BuiltUnitCardDefinition::new(),
            }
        }

        pub fn build(self) -> Box<dyn UnitCardDefinition> {
            Box::new(self.inner)
        }

        pub fn title(&mut self, title: impl AsRef<str>) -> &mut Self {
            self.inner.title = title.as_ref().to_string();
            self
        }
    }

    struct BuiltUnitCardDefinition {
        title: String,
        cost: i32,
        text: String,
        flavor_text: String,
        attack: i32,
        health: i32,
        row_width: i32,
        placeable_at: Position,
        is_defender: bool,
        is_hidden: bool,
        pre_action_event: Option<Box<dyn UponEventAction>>,
        post_action_event: Option<Box<dyn UponEventAction>>,
        passive_effect: Option<Box<dyn PassiveEffectDefinition>>,
        upon_summon: Box<dyn UponSummonAction>,
        upon_death: Box<dyn UponSummonAction>,
        upon_receive_damage: Box<dyn UponSummonAction>,
        upon_turn_start: Box<dyn UponSummonAction>,
        upon_turn_end: Box<dyn UponSummonAction>,
        upon_companion_damaged: Box<dyn UponSummonAction>,
    }

    impl BuiltUnitCardDefinition {
        pub(crate) fn new() -> Self {
            Self {
                title: todo!(),
                cost: todo!(),
                text: todo!(),
                flavor_text: todo!(),
                attack: todo!(),
                health: todo!(),
                row_width: todo!(),
                placeable_at: todo!(),
                is_defender: todo!(),
                is_hidden: todo!(),
                pre_action_event: todo!(),
                post_action_event: todo!(),
                passive_effect: todo!(),
                upon_summon: todo!(),
                upon_death: todo!(),
                upon_receive_damage: todo!(),
                upon_turn_start: todo!(),
                upon_turn_end: todo!(),
                upon_companion_damaged: todo!(),
            }
        }
    }

    impl std::fmt::Debug for BuiltUnitCardDefinition {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BuiltUnitCardDefinition")
                .field("title", &self.title)
                .field("cost", &self.cost)
                .field("text", &self.text)
                .field("flavor_text", &self.flavor_text)
                .field("attack", &self.attack)
                .field("health", &self.health)
                .field("row_width", &self.row_width)
                .field("placeable_at", &self.placeable_at)
                .field("is_defender", &self.is_defender)
                .field("is_hidden", &self.is_hidden)
                .finish()
        }
    }

    impl CardDefinition for BuiltUnitCardDefinition {
        fn title(&self) -> &str {
            todo!()
        }

        fn cost(&self) -> i32 {
            todo!()
        }

        fn text(&self) -> &str {
            todo!()
        }

        fn flavor_text(&self) -> &str {
            todo!()
        }
    }

    impl UnitCardDefinition for BuiltUnitCardDefinition {
        fn attack(&self) -> i32 {
            todo!()
        }

        fn health(&self) -> i32 {
            todo!()
        }

        fn row_width(&self) -> usize {
            todo!()
        }

        fn placeable_at(&self) -> Position {
            todo!()
        }

        fn upon_summon(&self) -> Box<dyn UponSummonAction> {
            Box::new(DoNothingAction)
        }

        fn upon_death(&self) -> Box<dyn UponDeathAction> {
            Box::new(DoNothingAction)
        }

        fn upon_receive_damage(&self) -> Box<dyn UponReceiveDamageAction> {
            Box::new(DoNothingAction)
        }

        fn upon_turn_start(&self) -> Box<dyn UponTurnStartAction> {
            Box::new(DoNothingAction)
        }

        fn upon_turn_end(&self) -> Box<dyn UponTurnEndAction> {
            Box::new(DoNothingAction)
        }

        fn upon_companion_damaged(&self) -> Box<dyn UponTurnEndAction> {
            Box::new(DoNothingAction)
        }

        fn passive_effect(&self) -> Option<Box<dyn PassiveEffectDefinition>> {
            None
        }

        fn pre_event_action(
            &self,
            _card_instance_id: CreatureInstanceId,
            _event: &GameEvent,
            _game_state: &GameState,
            _dispatcher: &mut EventDispatcher,
        ) -> Option<Box<dyn UponEventAction>> {
            None
        }

        fn post_event_action(
            &self,
            _card_instance_id: CreatureInstanceId,
            _event: &GameEvent,
            _game_state: &GameState,
            _dispatcher: &mut EventDispatcher,
        ) -> Option<Box<dyn UponEventAction>> {
            None
        }

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
}
