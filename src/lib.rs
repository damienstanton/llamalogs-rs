//! A Rust client for [llamalogs][1].
//! [1]: https://llamalogs.com/
//! Llama Logs is a brand new tool that turns distributed logs into a real time
//! interactive graph. It was created to help bring clarity to complex cloud
//! architectures

mod aggregator;
mod proxy;
mod types;

use aggregator::{add_log, add_stat, add_stat_avg, add_stat_max, start_timer};
use proxy::{collect_and_send, collect_and_send_blocking};
use surf::Exception;
use types::{GlobalState, Log, LogArgs, Stat, StatArgs};

fn process_log(global: GlobalState, mut log: Log) {
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

fn process_stat(global: GlobalState, mut stat: Stat) {
    if stat.account == "" {
        stat.account = global.account_key;
    }
    if stat.graph == "" {
        stat.graph == global.graph_name;
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
pub fn init(args: InitArgs) -> GlobalState {
    let mut g = GlobalState::default();
    g.account_key = args.account_key;
    g.graph_name = args.graph_name;
    g.is_dev_env = args.is_dev_env;
    g.is_disabled = args.is_disabled;
    g
}

/// Creates a new log for processing
pub fn log(global: GlobalState, args: LogArgs) {
    if global.is_disabled {
        return;
    }
    let log = args.to_log();
    process_log(global, log);
}

/// Creates a new point stat for processing
pub fn point_stat(global: GlobalState, args: StatArgs) {
    if global.is_disabled {
        return;
    }
    let mut stat = args.to_stat();
    stat.kind = "point";
    process_stat(global, stat);
}

/// Creates a new average stat for processing
pub fn average_stat(global: GlobalState, args: StatArgs) {
    if global.is_disabled {
        return;
    }
    let mut stat = args.to_stat();
    stat.kind = "average";
    process_stat(global, stat);
}

/// Creates a new max stat for processing
pub fn max_stat(global: GlobalState, args: StatArgs) {
    if global.is_disabled {
        return;
    }
    let mut stat = args.to_stat();
    stat.kind = "max";
    process_stat(global, stat);
}

/// Calls a blocking send of the current collection of logs and stats
pub fn force_send(global: GlobalState) -> Result<(), Exception> {
    if global.is_disabled {
        ()
    }
    collect_and_send_blocking(global);
    Ok(())
}
