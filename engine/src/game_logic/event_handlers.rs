mod add_buff_to_card_instance_handler;
mod add_card_to_hand_event_handler;
mod attack_event_handler;
mod creature_deals_damage_handler;
mod creature_destroyed_handler;
mod creature_healed_event_handler;
mod creature_set_event_handler;
mod creature_takes_damage_handler;
mod draw_card_event_handler;
mod end_turn_event_handler;
mod player_gain_mana_event_handler;
mod player_spend_mana_event_handler;
mod pos_takes_damage_event_handler;
mod start_game_event_handler;
mod summon_creature_from_hand_event_handler;
mod turn_start_event_handler;

pub use add_buff_to_card_instance_handler::AddBuffToCardInstanceHandler;
pub use add_card_to_hand_event_handler::AddCardToHandEventHandler;
pub use attack_event_handler::AttackEventHandler;
pub use creature_deals_damage_handler::CreatureDealsDamageHandler;
pub use creature_destroyed_handler::CreatureDestroyedEventHandler;
pub use creature_healed_event_handler::CreatureHealedEventHandler;
pub use creature_set_event_handler::CreatureSetEventHandler;
pub use creature_takes_damage_handler::CreatureTakesDamageHandler;
pub use draw_card_event_handler::DrawCardEventHandler;
pub use end_turn_event_handler::EndTurnEventHandler;
pub use player_gain_mana_event_handler::PlayerGainManaEventHandler;
pub use player_spend_mana_event_handler::PlayerSpendManaEventHandler;
pub use pos_takes_damage_event_handler::PosTakesDamageHandler;
pub use start_game_event_handler::StartGameEventHandler;
pub use summon_creature_from_hand_event_handler::SummonCreatureFromHandEventHandler;
pub use turn_start_event_handler::TurnStartHandler;

use super::events::Event;
use crate::game_state::game_state::GameState;

use async_trait::async_trait;

#[async_trait]
pub trait EventHandler {
    type Event: Event;
    async fn handle(
        &self,
        event: &Self::Event,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    );
}
