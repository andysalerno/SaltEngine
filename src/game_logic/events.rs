mod attack;
mod end_turn;
mod summon;

pub use attack::AttackEvent;
pub use end_turn::EndTurnEvent;
pub use summon::SummonCreatureEvent;

pub trait Event {}

#[derive(Debug)]
pub enum GameEvent {
    Attack(AttackEvent),
    EndTurn(EndTurnEvent),
    Summon(SummonCreatureEvent),
}
