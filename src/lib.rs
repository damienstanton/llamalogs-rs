//! A Rust client for [llamalogs][1].
//! [1]: https://llamalogs.com/
//! Llama Logs is a brand new tool that turns distributed logs into a real time
//! interactive graph. It was created to help bring clarity to complex cloud
//! architectures

mod aggregator;
mod proxy;
mod types;

use aggregator::{add_log, add_stat, start_timer};
use proxy::collect_and_send_blocking;
use types::{GlobalState, LlamaError, Log, LogArgs, Stat, StatArgs};

fn process_log(global: GlobalState, mut log: Log) {
    if log.account == "" {
        log.account = global.read().unwrap().account_key;
    }
    if log.graph == "" {
        log.graph = global.read().unwrap().graph_name;
    }
    if log.sender == "" || log.receiver == "" || log.account == "" || log.graph == "" {
        return;
    }
    add_log(global, log);
}

fn process_stat(global: GlobalState, mut stat: Stat) {
    if stat.account == "" {
        stat.account = global.read().unwrap().account_key;
    }
    if stat.graph == "" {
        stat.graph = global.read().unwrap().graph_name;
    }
    add_stat(global, stat);
}

// Public API
// ----------
pub struct InitArgs {
    graph_name: &'static str,
    account_key: &'static str,
    is_dev_env: bool,
    is_disabled: bool,
}

/// Creates the global state object used throughout the crate.
pub async fn init(args: InitArgs) -> GlobalState {
    let g = GlobalState::default();
    g.write().unwrap().account_key = args.account_key;
    g.write().unwrap().graph_name = args.graph_name;
    g.write().unwrap().is_dev_env = args.is_dev_env;
    g.write().unwrap().is_disabled = args.is_disabled;

    if !args.is_disabled {
        start_timer(&g).await;
    }
    g
}

/// Creates a new log for processing
pub fn log(global: GlobalState, args: LogArgs) {
    if global.read().unwrap().is_disabled {
        return;
    }
    let log = args.to_log();
    process_log(global, log);
}

/// Creates a new point stat for processing
pub fn point_stat(global: GlobalState, args: StatArgs) {
    if global.read().unwrap().is_disabled {
        return;
    }
    let mut stat = args.to_stat();
    stat.kind = "point";
    process_stat(global, stat);
}

/// Creates a new average stat for processing
pub fn average_stat(global: GlobalState, args: StatArgs) {
    if global.read().unwrap().is_disabled {
        return;
    }
    let mut stat = args.to_stat();
    stat.kind = "average";
    process_stat(global, stat);
}

/// Creates a new max stat for processing
pub fn max_stat(global: GlobalState, args: StatArgs) {
    if global.read().unwrap().is_disabled {
        return;
    }
    let mut stat = args.to_stat();
    stat.kind = "max";
    process_stat(global, stat);
}

/// Calls a blocking send of the current collection of logs and stats
pub fn force_send(global: GlobalState) -> Result<(), LlamaError> {
    if global.read().unwrap().is_disabled {
        ()
    }
    match collect_and_send_blocking(&global) {
        Ok(_) => Ok(()),
        Err(_) => Err(LlamaError::NetError()),
    }
}
