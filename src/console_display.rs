use crate::{
    game_runner::GameDisplay,
    game_state::{
        board::{BoardSlot, RowId},
        GameState,
    },
};

pub struct ConsoleDisplay;

impl GameDisplay for ConsoleDisplay {
    fn display(&mut self, game_state: &GameState) {
        let s = to_string(game_state);

        println!("{}", s);
    }
}

fn to_string(game_state: &GameState) -> String {
    let mut result = String::new();

    result.push_str(&row_to_string(
        game_state
            .board()
            .player_row(game_state.player_b_id(), RowId::BackRow),
        0,
        false,
    ));

    result.push('\n');
    result.push_str(&row_to_string(
        game_state
            .board()
            .player_row(game_state.player_b_id(), RowId::FrontRow),
        6,
        true,
    ));

    if game_state.cur_player_id() == game_state.player_b_id() {
        result.push_str("    <--- Player turn");
    }

    result.push('\n');
    result.push('\n');
    result.push_str(&row_to_string(
        game_state
            .board()
            .player_row(game_state.player_a_id(), RowId::FrontRow),
        12,
        false,
    ));

    result.push('\n');
    result.push_str(&row_to_string(
        game_state
            .board()
            .player_row(game_state.player_a_id(), RowId::BackRow),
        18,
        true,
    ));
    if game_state.cur_player_id() == game_state.player_a_id() {
        result.push_str("    <--- Player turn");
    }
    result.push('\n');

    let mana = game_state.player_mana(game_state.cur_player_id());
    result.push_str(&format!("Available mana: {}", mana));

    result
}

fn row_to_string(row: &[BoardSlot], start_index: usize, index_after: bool) -> String {
    let mut result = String::new();

    if !index_after {
        for i in 0..row.len() {
            let c = get_alpha_char(i + start_index);
            result.push_str(&format!("  {}  ", c));
        }
        result.push_str("\n");
    }

    let mut row_iter = row.iter();

    // For every slot in the row...
    while let Some(slot) = row_iter.next() {
        // If the slot contains a card instance...
        if let Some(card) = slot.maybe_creature() {
            let width = card.width();
            result.push_str(&format!("[{}/{}", card.attack(), card.health()));

            for _ in 1..width {
                let _ = row_iter.next();
                result.push_str("-----");
            }

            result.push_str("]");

        // If the slot is empty...
        } else {
            result.push_str("|   |");
        }
    }

    if index_after {
        result.push_str("\n");

        for i in 0..row.len() {
            let c = get_alpha_char(i + start_index);
            result.push_str(&format!("  {}  ", c));
        }
    }

    result
}

fn get_alpha_char(index: usize) -> char {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .nth(index)
        .expect("Expected index to be within range")
}
