use std::borrow::Borrow;

use crate::game_logic::{cards::UnitCardDefinition, BuffInstanceId, PassiveEffectInstance};
use crate::{game_logic::Buff, id::Id};

use super::{board::BoardPos, MakePlayerView};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UnitCardInstanceId(Id);

impl UnitCardInstanceId {
    pub fn new() -> Self {
        Self(Id::new())
    }
}

#[derive(Debug, Clone)]
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

    pub fn state(&self) -> Option<&InstanceState> {
        self.state.as_ref()
    }

    pub fn set_state(&mut self, state: Option<InstanceState>) {
        self.state = state;
    }
}

pub struct UnitCardInstancePlayerView {
    definition: Box<dyn UnitCardDefinition>,
    buffs: Vec<Box<dyn Buff>>,
    passive_effect: Option<PassiveEffectInstance>,
    id: UnitCardInstanceId,
    attack: i32,
    health: i32,
    width: usize,
    state: Option<InstanceState>,
}

impl MakePlayerView for UnitCardInstance {
    type TOut = UnitCardInstancePlayerView;

    fn player_view(&self) -> UnitCardInstancePlayerView {
        let def_player_view = self.definition.player_view();

        todo!()
    }
}
