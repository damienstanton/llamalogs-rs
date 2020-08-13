use crate::{
    args::{InitArgs, LogArgs},
    state::GlobalState,
};

/// Sets the accountKey and graphName to be used as defaults for all future
/// calls. Also starts the timer to automatically send data into the Llama Logs
/// server on a recurring basis.
pub fn init(args: InitArgs, state: &mut GlobalState) {
    let mut local = state.lock().unwrap(); // TODO: handle this null case
    local.graph_name = args.graph_name;
    local.account_key = args.account_key;
    local.is_dev_env = args.is_dev_env;
    local.is_disabled = args.is_disabled;
    if !local.is_disabled {
        // TODO: start_timer
    }
}

/// Logs an event that will be sent to the visual Llama Log graph on the
/// website. Logs are aggregated client side and then sent in as a batch to the
/// server on a repeating interval.
pub async fn log(args: LogArgs, state: &mut GlobalState) {
    let local = state.lock().unwrap(); // TODO: handle this null case
    if local.is_disabled {
        return;
    }
    let log = args.to_log().await;
    // TODO: process_log
}

/// A synchronous method to make an https request to send the aggregated logs
/// and stats into the Llama Logs server. This should be used to send in data in
/// cases where the interval timer might not be called. Such as at the end of a
/// cloud function, or other short lived processes. Each Llama Logs account has
/// rate limits so forcing sends in a loop will trigger the limits.
pub fn force_send() {}

fn point_stat() {}

fn avg_stat() {}

fn max_stat() {}

fn process_stat() {}

fn process_log() {}
