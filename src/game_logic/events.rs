mod add_card_to_hand_event;
mod attack;
mod creature_deals_damage_event;
mod creature_destroyed;
mod creature_takes_damage_event;
mod draw_card;
mod end_turn;
mod start_game_event;
mod summon;
mod turn_start_event;

pub use add_card_to_hand_event::AddCardToHandEvent;
pub use attack::AttackEvent;
pub use creature_deals_damage_event::CreatureDealsDamageEvent;
pub use creature_destroyed::CreatureDestroyedEvent;
pub use creature_takes_damage_event::CreatureTakesDamageEvent;
pub use draw_card::DrawCardEvent;
pub use end_turn::EndTurnEvent;
pub use start_game_event::StartGameEvent;
pub use summon::SummonCreatureEvent;
pub use turn_start_event::TurnStartEvent;

pub trait Event: Into<GameEvent> {}

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
    DrawCard(DrawCardEvent),
    AddCardToHand(AddCardToHandEvent),
    StartGame(StartGameEvent),
}
