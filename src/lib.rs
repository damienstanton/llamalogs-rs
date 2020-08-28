//! A Rust client for [llamalogs][1].
//! [1]: https://llamalogs.com/
//! Llama Logs is a brand new tool that turns distributed logs into a real time
//! interactive graph. It was created to help bring clarity to complex cloud
//! architectures

mod aggregator;
mod proxy;
mod types;

fn process_log() {}
fn process_stat() {}

// Public API
// ----------
pub use types::{GlobalState, LogArgs, StatArgs};

pub struct InitArgs {
    graph_name: &'static str,
    account_key: &'static str,
    is_dev_env: bool,
    is_disabled: bool,
}

pub fn init(args: InitArgs) -> GlobalState {
    let mut g = GlobalState::default();
    g.account_key = args.account_key;
    g.graph_name = args.graph_name;
    g.is_dev_env = args.is_dev_env;
    g.is_disabled = args.is_disabled;
    g
}

pub fn log(global: &GlobalState, args: LogArgs) {
    if global.is_disabled {
        return;
    }
    let log = args.to_log();
}

pub fn point_stat(global: &GlobalState, args: StatArgs) {}
pub fn average_stat(global: &GlobalState, args: StatArgs) {}
pub fn max_stat(global: &GlobalState, args: StatArgs) {}
pub fn force_send(global: &GlobalState) {}
