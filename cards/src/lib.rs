#![deny(clippy::all, nonstandard_style, future_incompatible)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::needless_pass_by_value,
    clippy::module_name_repetitions,
    clippy::unused_self,
    clippy::cast_lossless,
    clippy::similar_names,
    clippy::module_inception,
    dead_code
)]

// mod attack_dog;
// mod emotional_support_dog;
// mod fraidy_cat;
// mod grandma_the_soother;
// mod indoor_cat;
// mod outdoor_cat;
// mod pawn;
// mod popcorn_vendor;
// mod priest_of_the_lowland;
// mod really_big_rock;
// mod rickety_cannon;
// mod sleeping_dog;

// pub use attack_dog::AttackDog;
// pub use emotional_support_dog::EmotionalSupportDog;
// pub use fraidy_cat::FraidyCat;
// pub use grandma_the_soother::GrandmaTheSoother;
// pub use indoor_cat::IndoorCat;
// pub use outdoor_cat::OutdoorCat;
// pub use pawn::Pawn;
// pub use popcorn_vendor::PopcornVendor;
// pub use priest_of_the_lowland::PriestOfTheLowland;
// pub use really_big_rock::ReallyBigRock;
// pub use rickety_cannon::RicketyCannon;
// pub use sleeping_dog::SleepingDog;

// #[cfg(test)]
// mod tests {
//     use std::sync::Arc;

//     use async_trait::async_trait;
//     use mockall::mock;
//     use protocol::{
//         entities::{BoardPos, PlayerId},
//         from_server::{Notification, VisualEvent},
//     };
//     use salt_engine::{
//         cards::UnitCardDefinition,
//         game_agent::{ClientNotifier, Prompter},
//         game_logic::EventDispatcher,
//         game_state::{Deck, GameState, GameStatePlayerView},
//     };

//     use crate::Pawn;

//     mock! {
//         pub(crate) TestPrompter {}
//         impl Prompter for TestPrompter {
//             fn prompt_slot(&self) -> BoardPos;
//             fn prompt_player_slot(&self) -> BoardPos;
//             fn prompt_opponent_slot(&self) -> BoardPos;
//             fn prompt_creature_pos(&self) -> BoardPos;
//             fn prompt_player_creature_pos(&self) -> BoardPos;
//             fn prompt_opponent_creature_pos(&self) -> BoardPos;
//         }
//     }

//     struct TestClientNotifier;

//     #[async_trait]
//     impl ClientNotifier for TestClientNotifier {
//         async fn notify(&self, _event: Notification) {
//             // Doing nothing for tests
//         }
//     }

//     pub fn make_test_state() -> GameState {
//         let player_a_deck = Deck::new(vec![Pawn.make_instance()]);
//         let player_b_deck = Deck::new(vec![Pawn.make_instance()]);

//         let mut state = GameState::initial_state(
//             PlayerId::new(),
//             player_a_deck,
//             PlayerId::new(),
//             player_b_deck,
//         );

//         state.raise_mana_limit(state.player_a_id(), 10);
//         state.raise_mana_limit(state.player_b_id(), 10);
//         state.refresh_player_mana(state.player_a_id());
//         state.refresh_player_mana(state.player_b_id());

//         state
//     }

//     // pub fn make_dispatcher(player_a_id: PlayerId, player_b_id: PlayerId) -> EventDispatcher {
//     //     let notifier_a = Arc::new(TestClientNotifier);
//     //     let notifier_b = Arc::new(TestClientNotifier);

//     //     let prompter_a = Arc::new(MockTestPrompter::new());
//     //     let prompter_b = Arc::new(MockTestPrompter::new());

//     //     EventDispatcher::new(
//     //         notifier_a,
//     //         prompter_a,
//     //         player_a_id,
//     //         notifier_b,
//     //         prompter_b,
//     //         player_b_id,
//     //     )
//     // }
// }
