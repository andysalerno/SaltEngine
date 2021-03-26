mod add_card_to_hand_event;
mod attack;
mod creature_deals_damage_event;
mod creature_destroyed;
mod creature_healed_event;
mod creature_set_event;
mod creature_takes_damage_event;
mod draw_card;
mod end_turn;
mod player_gain_mana;
mod player_spend_mana;
mod pos_takes_damage_event;
mod start_game_event;
mod summon_creature_from_hand_event;
mod turn_start_event;

pub use add_card_to_hand_event::AddCardToHandEvent;
pub use attack::AttackEvent;
pub use creature_deals_damage_event::CreatureDealsDamageEvent;
pub use creature_destroyed::CreatureDestroyedEvent;
pub use creature_healed_event::CreatureHealedEvent;
pub use creature_set_event::CreatureSetEvent;
pub use creature_takes_damage_event::CreatureTakesDamageEvent;
pub use draw_card::DrawCardEvent;
pub use end_turn::EndTurnEvent;
pub use player_gain_mana::PlayerGainManaEvent;
pub use player_spend_mana::PlayerSpendManaEvent;
pub use pos_takes_damage_event::PosTakesDamageEvent;
pub use start_game_event::StartGameEvent;
pub use summon_creature_from_hand_event::SummonCreatureFromHandEvent;
pub use turn_start_event::TurnStartEvent;

use crate::game_state::GameStateView;

pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub trait Event: Into<GameEvent> {
    fn validate<'a, G>(&self, _game_state: &'a G) -> Result
    where
        G: GameStateView<'a>,
    {
        Ok(())
    }
}

#[derive(Debug)]
pub enum GameEvent {
    Attack(AttackEvent),
    EndTurn(EndTurnEvent),
    Summon(CreatureSetEvent),
    CreatureDealsDamage(CreatureDealsDamageEvent),
    CreatureTakesDamage(CreatureTakesDamageEvent),
    CreatureDestroyed(CreatureDestroyedEvent),
    TurnStart(TurnStartEvent),
    DrawCard(DrawCardEvent),
    AddCardToHand(AddCardToHandEvent),
    StartGame(StartGameEvent),
    GainMana(PlayerGainManaEvent),
    SpendMana(PlayerSpendManaEvent),
    SummonCreatureFromHand(SummonCreatureFromHandEvent),
    PosTakesDamage(PosTakesDamageEvent),
    CreatureHealed(CreatureHealedEvent),
}
