//! A Rust client for [llamalogs][1].
//! [1]: https://llamalogs.com/
//! Llama Logs is a brand new tool that turns distributed logs into a real time
//! interactive graph. It was created to help bring clarity to complex cloud
//! architectures

mod logger;
mod types;

// Public API
// ----------
pub use logger::Logger;
pub use types::{LogArgs, StatArgs};

pub struct InitArgs {
    pub graph_name: &'static str,
    pub account_key: &'static str,
    pub is_dev_env: bool,
    pub is_disabled: bool,
}

/// Creates the global state object used throughout the crate.
pub fn init(args: InitArgs) -> Logger {
    let mut g = Logger::default();
    g.account_key = args.account_key;
    g.graph_name = args.graph_name;
    g.is_dev_env = args.is_dev_env;
    g.is_disabled = args.is_disabled;
    g
}
