pub mod connection;
pub mod messages;
mod network_prompter;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
