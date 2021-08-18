mod console_agent;
mod console_display;

use console_agent::ConsoleAgent;
use env_logger::Env;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let agent = Box::new(ConsoleAgent::new());
}
