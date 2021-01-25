mod attack;
mod end_turn;

pub use attack::AttackEvent;
pub use end_turn::EndTurnEvent;

pub trait Event {}

pub enum GameEvent {
    Attack(AttackEvent),
    EndTurn(EndTurnEvent),
}
