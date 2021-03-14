mod console_display;
mod game_agent;
mod game_logic;
mod game_runner;
pub mod game_state;
pub mod id;

use console_display::ConsoleDisplay;
use game_agent::console_agent::ConsoleAgent;

fn main() {
    let player_a = Box::new(ConsoleAgent::new());
    let player_b = Box::new(ConsoleAgent::new());
    let display = Box::new(ConsoleDisplay);

    let mut runner = game_runner::GameRunner::new(player_a, player_b, display);

    runner.run_game();
}
