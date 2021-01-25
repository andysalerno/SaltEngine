use crate::{
    game_runner::GameDisplay,
    game_state::{GameState, UnitCardBoardInstance},
    id::HasId,
};

pub struct ConsoleDisplay;

impl GameDisplay for ConsoleDisplay {
    fn display(&mut self, game_state: &GameState) {
        let s = to_string(game_state);

        println!("displaying...");
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
    result.push_str(&row_to_string(game_state.board().player_side().front_row()));
    result.push('\n');
    result.push_str(&row_to_string(game_state.board().player_side().back_row()));

    result
}

fn row_to_string(row: &[Option<UnitCardBoardInstance>]) -> String {
    let mut result = String::new();

    let mut row_iter = row.iter();

    while let Some(maybe_card) = row_iter.next() {
        if let Some(card) = maybe_card {
            // Starting a new card
            let mut id = card.id();
            result.push_str(&format!("[{}/{}", card.attack(), card.health()));

            while let Some(Some(next_card)) = row_iter.next() {
                if next_card.id() == id {
                    // Add empty space for the same card...
                    result.push_str("   ");
                } else {
                    // But if it changes, start a new card
                    id = next_card.id();
                    result.push_str("]");
                    result.push_str(&format!("[{}/{}", card.attack(), card.health()));
                }
            }

            result.push_str("]");
        } else {
            // Starting new chunk of empty slots
            result.push_str("|   |");
        }
    }

    result
}
