use protocol::entities::RowId;
use salt_engine::game_state::{
    board::{BoardSlotPlayerView, BoardView},
    GameStatePlayerView, GameStateView,
};

pub struct ConsoleDisplay;

impl ConsoleDisplay {
    pub(crate) fn display(&mut self, game_state: &GameStatePlayerView) {
        let s = to_string(game_state);

        println!("{}", s);
    }
}

fn to_string(game_state: &GameStatePlayerView) -> String {
    let mut result = String::new();

    result.push_str(&row_to_string(
        game_state
            .board()
            .player_row(game_state.opponent_id(), RowId::BackRow),
        0,
        false,
    ));
    let player_b_health = game_state.board().hero(game_state.opponent_id()).health();
    result.push_str(&format!("    (Y) Health: {}", player_b_health));

    result.push('\n');
    result.push_str(&row_to_string(
        game_state
            .board()
            .player_row(game_state.opponent_id(), RowId::FrontRow),
        6,
        true,
    ));

    if game_state.cur_player_turn() == game_state.opponent_id() {
        result.push_str("    <--- Player turn");
    }

    result.push('\n');
    result.push('\n');
    result.push_str(&row_to_string(
        game_state
            .board()
            .player_row(game_state.player_id(), RowId::FrontRow),
        12,
        false,
    ));
    let player_a_health = game_state.board().hero(game_state.player_id()).health();
    result.push_str(&format!("    (Z) Health: {}", player_a_health));

    result.push('\n');
    result.push_str(&row_to_string(
        game_state
            .board()
            .player_row(game_state.player_id(), RowId::BackRow),
        18,
        true,
    ));
    if game_state.cur_player_turn() == game_state.player_id() {
        result.push_str("    <--- Player turn");
    }
    result.push('\n');

    let mana = game_state.player_mana(game_state.cur_player_turn());
    result.push_str(&format!("Available mana: {}", mana));

    result
}

fn row_to_string(row: &[BoardSlotPlayerView], start_index: usize, index_after: bool) -> String {
    let mut result = String::new();

    if !index_after {
        for i in 0..row.len() {
            let c = get_alpha_char(i + start_index);
            result.push_str(&format!("  {}  ", c));
        }
        result.push('\n');
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

            result.push(']');

        // If the slot is empty...
        } else {
            result.push_str("|   |");
        }
    }

    if index_after {
        result.push('\n');

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
