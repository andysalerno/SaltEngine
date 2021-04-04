use super::{board::BoardPos, MakePlayerView, PlayerId};
use crate::game_logic::{
    cards::{
        player_view::UnitCardDefinitionPlayerView, UnitCardDefinition, UnitCardDefinitionView,
    },
    BuffInstanceId, BuffPlayerView, PassiveEffectInstance, PassiveEffectInstancePlayerView,
};
use crate::game_logic::{Buff, BuffView, PassiveEffectView};
use crate::id::Id;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

pub trait UnitCardInstanceView<'a> {
    type DefinitionView: ?Sized + UnitCardDefinitionView;
    type Buffs: BuffView;
    type PassiveEffect: PassiveEffectView;

    fn definition(&'a self) -> &'a Self::DefinitionView;
    fn buffs(&'a self) -> &'a Vec<Self::Buffs>;
    fn passive_effect(&self) -> Option<&Self::PassiveEffect>;
    fn id(&self) -> UnitCardInstanceId;
    fn attack(&self) -> i32;
    fn health(&self) -> i32;
    fn width(&self) -> usize;
    fn state(&self) -> Option<InstanceState>;
}

impl<'a> UnitCardInstanceView<'a> for UnitCardInstance {
    type DefinitionView = dyn UnitCardDefinition;
    type Buffs = Box<dyn Buff>;
    type PassiveEffect = PassiveEffectInstance;

    fn definition(&'a self) -> &'a (dyn UnitCardDefinition + 'static) {
        self.definition.as_ref()
    }

    fn buffs(&'a self) -> &'a Vec<Box<dyn Buff>> {
        &self.buffs
    }

    fn passive_effect(&self) -> Option<&PassiveEffectInstance> {
        self.passive_effect.as_ref()
    }

    fn id(&self) -> UnitCardInstanceId {
        self.id()
    }

    fn attack(&self) -> i32 {
        self.attack()
    }

    fn health(&self) -> i32 {
        self.health()
    }

    fn width(&self) -> usize {
        self.width()
    }

    fn state(&self) -> Option<InstanceState> {
        self.state()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnitCardInstanceId(Id);

impl UnitCardInstanceId {
    pub fn new() -> Self {
        Self(Id::new())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InstanceState {
    Pos(BoardPos),
    CreatureInstanceId(UnitCardInstanceId),
}

#[derive(Debug)]
pub struct UnitCardInstance {
    definition: Box<dyn UnitCardDefinition>,
    buffs: Vec<Box<dyn Buff>>,
    passive_effect: Option<PassiveEffectInstance>,
    id: UnitCardInstanceId,
    attack: i32,
    health: i32,
    width: usize,
    state: Option<InstanceState>,
}

impl UnitCardInstance {
    pub fn new(definition: Box<dyn UnitCardDefinition>) -> Self {
        let id = UnitCardInstanceId::new();

        let passive_effect = definition
            .passive_effect()
            .map(|e| PassiveEffectInstance::new(e, id));

        Self {
            attack: definition.attack(),
            health: definition.health(),
            width: definition.row_width(),
            definition,
            passive_effect,
            buffs: Vec::new(),
            id,
            state: None,
        }
    }

    pub fn attack(&self) -> i32 {
        let attack_buf: i32 = self.buffs().iter().map(|b| b.attack_amount()).sum();

        self.attack + attack_buf
    }

    pub fn health(&self) -> i32 {
        let health_buf: i32 = self.buffs().iter().map(|b| b.health_amount()).sum();
        self.health + health_buf
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn buffs(&self) -> &[Box<dyn Buff>] {
        self.buffs.as_slice()
    }

    pub fn definition(&self) -> &dyn UnitCardDefinition {
        self.definition.borrow()
    }

    pub fn take_damage(&mut self, damage_amount: usize) {
        self.health -= damage_amount as i32;
    }

    /// Increases health by heal_amount, but not beyond the starting health from the
    /// creature's definition.
    pub fn receive_heal(&mut self, heal_amount: usize) {
        let starting_health = self.health();
        let max_health = self.definition().health();

        let new_health = std::cmp::min(max_health, starting_health + heal_amount as i32);

        self.health = new_health;
    }

    pub fn add_buff(&mut self, buff: Box<dyn Buff>) {
        self.buffs.push(buff);
    }

    pub fn remove_buff(&mut self, buff_id: BuffInstanceId) {
        self.buffs.retain(|i| i.instance_id() != buff_id);
    }

    pub fn passive_effect_instance(&self) -> Option<&PassiveEffectInstance> {
        self.passive_effect.borrow().as_ref()
    }

    pub fn id(&self) -> UnitCardInstanceId {
        self.id
    }

    pub fn state(&self) -> Option<InstanceState> {
        self.state
    }

    pub fn set_state(&mut self, state: Option<InstanceState>) {
        self.state = state;
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UnitCardInstancePlayerView {
    definition: UnitCardDefinitionPlayerView,
    buffs: Vec<BuffPlayerView>,
    passive_effect: Option<PassiveEffectInstancePlayerView>,
    id: UnitCardInstanceId,
    attack: i32,
    health: i32,
    width: usize,
    state: Option<InstanceState>,
}

impl UnitCardInstancePlayerView {
    pub fn id(&self) -> UnitCardInstanceId {
        self.id
    }

    pub fn definition(&self) -> &UnitCardDefinitionPlayerView {
        &self.definition
    }

    pub fn buffs(&self) -> &Vec<BuffPlayerView> {
        &self.buffs
    }

    pub fn passive_effect(&self) -> Option<&PassiveEffectInstancePlayerView> {
        self.passive_effect.as_ref()
    }

    pub fn attack(&self) -> i32 {
        self.attack
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn state(&self) -> Option<InstanceState> {
        self.state
    }
}

impl MakePlayerView for UnitCardInstance {
    type TOut = UnitCardInstancePlayerView;

    fn player_view(&self, player_viewing: PlayerId) -> UnitCardInstancePlayerView {
        let definition = self.definition.player_view(player_viewing);
        let buffs = self
            .buffs
            .iter()
            .map(|b| b.player_view(player_viewing))
            .collect();

        let passive_effect = self
            .passive_effect
            .as_ref()
            .map(|p| p.player_view(player_viewing));

        UnitCardInstancePlayerView {
            definition,
            buffs,
            passive_effect,
            id: self.id(),
            attack: self.attack(),
            health: self.health(),
            width: self.width(),
            state: self.state.clone(),
        }
    }
}

impl<'a> UnitCardInstanceView<'a> for UnitCardInstancePlayerView {
    type DefinitionView = UnitCardDefinitionPlayerView;
    type Buffs = BuffPlayerView;
    type PassiveEffect = PassiveEffectInstancePlayerView;

    fn definition(&self) -> &UnitCardDefinitionPlayerView {
        UnitCardInstancePlayerView::definition(self)
    }

    fn buffs(&'a self) -> &'a Vec<Self::Buffs> {
        UnitCardInstancePlayerView::buffs(self)
    }

    fn passive_effect(&self) -> Option<&Self::PassiveEffect> {
        UnitCardInstancePlayerView::passive_effect(self)
    }

    fn id(&self) -> UnitCardInstanceId {
        UnitCardInstancePlayerView::id(self)
    }

    fn attack(&self) -> i32 {
        UnitCardInstancePlayerView::attack(self)
    }

    fn health(&self) -> i32 {
        UnitCardInstancePlayerView::health(self)
    }

    fn width(&self) -> usize {
        UnitCardInstancePlayerView::width(self)
    }

    fn state(&self) -> Option<InstanceState> {
        UnitCardInstancePlayerView::state(self)
    }
}
