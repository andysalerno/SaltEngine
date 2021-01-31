mod attack;
mod creature_deals_damage_event;
mod creature_takes_damage_event;
mod end_turn;
mod summon;

pub use attack::AttackEvent;
pub use creature_deals_damage_event::CreatureDealsDamageEvent;
pub use creature_takes_damage_event::CreatureTakesDamageEvent;
pub use end_turn::EndTurnEvent;
pub use summon::SummonCreatureEvent;

pub trait Event {}

#[derive(Debug)]
pub enum GameEvent {
    Attack(AttackEvent),
    EndTurn(EndTurnEvent),
    Summon(SummonCreatureEvent),
    CreatureDealsDamage(CreatureDealsDamageEvent),
    CreatureTakesDamage(CreatureTakesDamageEvent),
}
