//! A Rust client for [llamalogs][1].
//! [1]: https://llamalogs.com/
//! Llama Logs is a brand new tool that turns distributed logs into a real time
//! interactive graph. It was created to help bring clarity to complex cloud
//! architectures
mod aggregator;
mod args;
mod llama;
mod log;
mod state;
mod statistics;

// Define the public API here
pub use args::{InitArgs, LogArgs, StatArgs};
pub use llama::{force_send, init, log};
pub use log::Log;
pub use state::Global;
pub use statistics::Stat;
