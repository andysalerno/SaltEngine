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
    use crate::{
        game_state::game_state::GameState,
        v2::{CreatureDefinitionId, CreatureInstance},
    };
    use protocol::entities::{BoardPos, EntityPosition, PlayerId, RowId};

    #[test]
    fn can_set_and_get_creature() {
        let player_a = PlayerId::new();
        let player_b = PlayerId::new();
        let mut game_state = GameState::new(player_a, player_b);

        let mut board = game_state.board_mut();
        let board_pos = BoardPos {
            player_id: PlayerId::new(),
            row_id: RowId::FrontRow,
            row_index: 5,
        };
        let creature = CreatureInstance::new_from_definition_id(CreatureDefinitionId::new());
        board.set_creature_at_pos(creature, board_pos);

        let creature = board.creature_at_pos(board_pos);

        assert!(creature.is_some());
    }
}
