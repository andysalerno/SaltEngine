mod attack_dog;
mod emotional_support_dog;
mod fraidy_cat;
mod indoor_cat;
mod outdoor_cat;
mod pawn;
mod popcorn_vendor;
mod priest_of_the_lowland;
mod really_big_rock;
mod rickety_cannon;
mod sleeping_dog;

pub use attack_dog::AttackDog;
pub use emotional_support_dog::EmotionalSupportDog;
pub use fraidy_cat::FraidyCat;
pub use indoor_cat::IndoorCat;
pub use outdoor_cat::OutdoorCat;
pub use pawn::Pawn;
pub use popcorn_vendor::PopcornVendor;
pub use priest_of_the_lowland::PriestOfTheLowland;
pub use really_big_rock::ReallyBigRock;
pub use rickety_cannon::RicketyCannon;
pub use sleeping_dog::SleepingDog;

#[cfg(test)]
mod tests {
    use mockall::{mock, predicate::*};
    use salt_engine::{
        game_agent::game_agent::{ClientNotifier, Prompter},
        game_logic::EventDispatcher,
        game_state::{board::BoardPos, Deck, GameState, GameStatePlayerView, PlayerId},
    };

    mock! {
        pub TestPrompter {}
        impl Prompter for TestPrompter {
            fn prompt_slot(&self, game_state: &GameStatePlayerView) -> BoardPos;
            fn prompt_player_slot(&self, game_state: &GameStatePlayerView) -> BoardPos;
            fn prompt_opponent_slot(&self, game_state: &GameStatePlayerView) -> BoardPos;
            fn prompt_creature_pos(&self, game_state: &GameStatePlayerView) -> BoardPos;
            fn prompt_player_creature_pos(&self, game_state: &GameStatePlayerView) -> BoardPos;
            fn prompt_opponent_creature_pos(&self, game_state: &GameStatePlayerView) -> BoardPos;
        }
    }

    pub fn make_test_state() -> GameState {
        let player_a_deck = Deck::new(Vec::new());
        let player_b_deck = Deck::new(Vec::new());

        let mut state = GameState::initial_state(
            PlayerId::new(),
            player_a_deck,
            PlayerId::new(),
            player_b_deck,
        );

        state.raise_mana_limit(state.player_a_id(), 10);
        state.raise_mana_limit(state.player_b_id(), 10);
        state.refresh_player_mana(state.player_a_id());
        state.refresh_player_mana(state.player_b_id());

        state
    }

    pub fn make_default_dispatcher() -> EventDispatcher {
        todo!()
        // let prompt_a = MockTestPrompter::new();
        // let prompt_b = MockTestPrompter::new();

        // make_dispatcher(prompt_a, PlayerId::new(), prompt_b, PlayerId::new())
    }

    pub fn make_dispatcher(
        prompter_a: impl ClientNotifier + 'static,
        player_a_id: PlayerId,
        prompter_b: impl ClientNotifier + 'static,
        player_b_id: PlayerId,
    ) -> EventDispatcher {
        todo!()
        // EventDispatcher::new(
        //     Box::new(prompter_a),
        //     player_a_id,
        //     Box::new(prompter_b),
        //     player_b_id,
        // )
    }
}
