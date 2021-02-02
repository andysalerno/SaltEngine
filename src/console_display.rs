use crate::{
    game_runner::GameDisplay,
    game_state::{GameState, UnitCardBoardInstance},
    id::HasId,
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
        game_state.board().opponent_side().back_row(),
    ));
    result.push('\n');
    result.push_str(&row_to_string(
        game_state.board().opponent_side().front_row(),
    ));

    result.push('\n');
    result.push('\n');

    result.push_str(&row_to_string(game_state.board().player_side().front_row()));
    result.push('\n');
    result.push_str(&row_to_string(game_state.board().player_side().back_row()));

    result
}

fn row_to_string(row: &[Option<UnitCardBoardInstance>]) -> String {
    let mut result = String::new();

    let mut row_iter = row.iter();

    // For every slot in the row...
    while let Some(maybe_card) = row_iter.next() {
        // If the slot contains a card instance...
        if let Some(card) = maybe_card {
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

    result
}
