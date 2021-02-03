mod attack;
mod creature_deals_damage_event;
mod creature_destroyed;
mod creature_takes_damage_event;
mod end_turn;
mod summon;
mod turn_start_event;

pub use attack::AttackEvent;
pub use creature_deals_damage_event::CreatureDealsDamageEvent;
pub use creature_destroyed::CreatureDestroyedEvent;
pub use creature_takes_damage_event::CreatureTakesDamageEvent;
pub use end_turn::EndTurnEvent;
pub use summon::SummonCreatureEvent;
pub use turn_start_event::TurnStartEvent;

pub trait Event {}

#[derive(Debug)]
pub enum GameEvent {
    Attack(AttackEvent),
    EndTurn(EndTurnEvent),
    Summon(SummonCreatureEvent),
    CreatureDealsDamage(CreatureDealsDamageEvent),
    CreatureTakesDamage(CreatureTakesDamageEvent),
    CreatureDestroyed(CreatureDestroyedEvent),
    TurnEnd(EndTurnEvent),
    TurnStart(TurnStartEvent),
}
