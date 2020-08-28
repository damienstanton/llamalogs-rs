use chrono::Utc;
use serde::Serialize;
use std::{collections::HashMap, sync::*}; // TODO: settle on a threadsafe container

#[derive(Debug)]
pub struct LogArgs {
    sender: &'static str,
    receiver: &'static str,
    message: &'static str,
    is_error: bool,
    account_key: &'static str,
    graph_name: &'static str,
}

impl LogArgs {
    pub fn to_log(self) -> Log {
        Log {
            sender: self.sender,
            receiver: self.receiver,
            message: self.message,
            timestamp: Utc::now().timestamp_millis(),
            is_initial_message: true,
            account: self.account_key,
            graph: self.graph_name,
            is_error: self.is_error,
            elapsed: 0i64,
        }
    }
}

#[derive(Debug)]
pub struct StatArgs {
    component: &'static str,
    name: &'static str,
    value: f64,
    account_key: &'static str,
    graph_name: &'static str,
}

impl StatArgs {
    pub fn to_stat(self) -> Stat {
        Stat {
            component: self.component,
            name: self.name,
            value: self.value,
            account: self.account_key,
            graph: self.graph_name,
            timestamp: Utc::now().timestamp_millis(),
            kind: "",
            count: 0i64,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Log {
    pub(crate) sender: &'static str,
    pub(crate) receiver: &'static str,
    pub(crate) timestamp: i64,
    pub(crate) message: &'static str,
    pub(crate) is_initial_message: bool,
    pub(crate) account: &'static str,
    pub(crate) graph: &'static str,
    pub(crate) is_error: bool,
    pub(crate) elapsed: i64,
}

impl Log {
    pub(crate) fn to_aggregate_log(self) -> AggregateLog {
        AggregateLog {
            log: self,
            account_key: "",
            graph_name: "",
            start_time: 0i64,
            initial_message_count: 0i64,
            error_message: "",
            count: 0i64,
            errors: 0i64,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub(crate) struct AggregateLog {
    pub(crate) log: Log,
    pub(crate) account_key: &'static str,
    pub(crate) graph_name: &'static str,
    pub(crate) start_time: i64,
    #[serde(rename(serialize = "initialMessageCount"))]
    pub(crate) initial_message_count: i64,
    #[serde(rename(serialize = "errorMessage"))]
    pub(crate) error_message: &'static str,
    pub(crate) count: i64,
    pub(crate) errors: i64,
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Stat {
    pub(crate) component: &'static str,
    pub(crate) name: &'static str,
    pub(crate) value: f64,
    pub(crate) kind: &'static str,
    pub(crate) timestamp: i64,
    pub(crate) account: &'static str,
    pub(crate) graph: &'static str,
    pub(crate) count: i64,
}

pub type GlobalState = State;
pub(crate) type LogData = HashMap<&'static str, HashMap<&'static str, AggregateLog>>;
pub(crate) type StatData = HashMap<&'static str, HashMap<&'static str, Stat>>;

#[derive(Debug, Default)]
pub struct State {
    pub(crate) is_dev_env: bool,
    pub(crate) is_disabled: bool,
    pub(crate) account_key: &'static str,
    pub(crate) graph_name: &'static str,
    pub(crate) aggregated_logs: LogData,
    pub(crate) aggregated_stats: StatData,
    pub(crate) timer_started: bool,
}

#[derive(Default, Debug, Serialize)]
pub(crate) struct LogRequest {
    pub(crate) account_key: &'static str,
    pub(crate) time_logs: Vec<AggregateLog>,
    pub(crate) time_stats: Vec<Stat>,
}
