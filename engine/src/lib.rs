#![deny(clippy::all, nonstandard_style, future_incompatible)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::needless_pass_by_value,
    clippy::module_name_repetitions,
    clippy::unused_self,
    clippy::cast_lossless,
    clippy::module_inception,
    clippy::missing_panics_doc,
    clippy::similar_names,
    clippy::cast_sign_loss,
    dead_code
)]

pub mod game_agent;
pub mod game_logic;
// pub mod game_runner;
pub mod game_state;
mod v2;

// pub use game_logic::cards;

#[cfg(test)]
mod tests {
    use protocol::entities::PlayerId;

    use crate::game_state::game_state::GameState;

    #[test]
    fn test() {
        let player_a = PlayerId::new();
        let player_b = PlayerId::new();
        let game_state = GameState::new(player_a, player_b);
    }
}
