mod console_agent;
mod console_display;

use console_agent::ConsoleAgent;
use env_logger::Env;
use salt_engine::game_agent::game_agent::GameAgent;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    smol::block_on(async {
        let make_agent =
            |player_id| Box::new(ConsoleAgent::new_with_id(player_id)) as Box<dyn GameAgent>;

        client::start(make_agent)
            .await
            .expect("Failed to start client.");
    })
}
