pub mod connection;
pub mod messages;
mod network_prompter;

/// A Result<T, E> where E is a boxed dyn error, plus Send and Sync.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
