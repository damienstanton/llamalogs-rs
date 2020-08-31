//! A Rust client for [llamalogs][1].
//! [1]: https://llamalogs.com/
//! Llama Logs is a brand new tool that turns distributed logs into a real time
//! interactive graph. It was created to help bring clarity to complex cloud
//! architectures

mod aggregator;
mod proxy;
mod types;

use aggregator::{add_log, add_stat};
use proxy::send_blocking;
use types::{GlobalState, LlamaError, Log, Stat, StatArgs};

fn process_log(global: &mut GlobalState, mut log: Log) {
    if log.account == "" {
        log.account = global.account_key;
    }
    if log.graph == "" {
        log.graph = global.graph_name;
    }
    if log.sender == "" || log.receiver == "" || log.account == "" || log.graph == "" {
        return;
    }
    add_log(global, log);
}

fn process_stat(global: &mut GlobalState, mut stat: Stat) {
    if stat.account == "" {
        stat.account = global.account_key;
    }
    if stat.graph == "" {
        stat.graph = global.graph_name;
    }
    add_stat(global, stat);
}

// Public API
// ----------
pub use aggregator::start_timer;
pub use proxy::collect_messages;
pub use types::LogArgs;
pub struct InitArgs {
    pub graph_name: &'static str,
    pub account_key: &'static str,
    pub is_dev_env: bool,
    pub is_disabled: bool,
}

/// Creates the global state object used throughout the crate.
pub fn init(args: InitArgs) -> GlobalState {
    let mut g = GlobalState::default();
    g.account_key = args.account_key;
    g.graph_name = args.graph_name;
    g.is_dev_env = args.is_dev_env;
    g.is_disabled = args.is_disabled;
    g
}

/// Creates a new log for processing
pub fn log(global: &mut GlobalState, args: LogArgs) {
    if global.is_disabled {
        return;
    }
    let log = args.to_log();
    process_log(global, log);
}

/// Creates a new point stat for processing
pub fn point_stat(global: &mut GlobalState, args: StatArgs) {
    if global.is_disabled {
        return;
    }
    let mut stat = args.to_stat();
    stat.kind = "point";
    process_stat(global, stat);
}

/// Creates a new average stat for processing
pub fn average_stat(global: &mut GlobalState, args: StatArgs) {
    if global.is_disabled {
        return;
    }
    let mut stat = args.to_stat();
    stat.kind = "average";
    process_stat(global, stat);
}

/// Creates a new max stat for processing
pub fn max_stat(global: &mut GlobalState, args: StatArgs) {
    if global.is_disabled {
        return;
    }
    let mut stat = args.to_stat();
    stat.kind = "max";
    process_stat(global, stat);
}

/// Calls a blocking send of the current collection of logs and stats
pub fn force_send(mut global: &mut GlobalState) -> Result<(), LlamaError> {
    if global.is_disabled {
        ()
    }
    match send_blocking(&mut global) {
        Ok(_) => Ok(()),
        Err(_) => Err(LlamaError::NetError()),
    }
}
