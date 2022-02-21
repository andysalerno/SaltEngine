#![deny(clippy::all, nonstandard_style, future_incompatible)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::needless_pass_by_value,
    clippy::unused_self,
    clippy::cast_lossless
)]

pub mod connection;
mod network_prompter;

/// A Result<T, E> where E is a boxed dyn error, plus Send and Sync.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
