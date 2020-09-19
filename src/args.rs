use crate::types::{Log, Stat};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Result};

#[derive(Debug, Serialize, Deserialize)]
/// A structure for constructing a new `Log` from a set of input arguments
pub struct LogArg {
    pub sender: &'static str,
    pub receiver: &'static str,
    pub message: &'static str,
    pub is_error: bool,
    pub account_key: &'static str,
    pub graph_name: &'static str,
}

impl LogArg {
    /// Create a `Log` instance from an input JSON of the type `&str`
    pub fn from_json(&self, input: &'static str) -> Result<Log> {
        Ok(from_str(input)?)
    }

    /// Create a new `Log`
    pub fn to_log(&self) -> Log {
        let ts = Utc::now().timestamp_millis();
        Log {
            sender: self.sender,
            receiver: self.receiver,
            message: self.message,
            timestamp: ts,
            is_initial_message: true,
            account_key: self.account_key,
            graph: self.graph_name,
            is_error: self.is_error,
            elapsed: 0,
            start_time: ts,
            initial_message_count: 0,
            error_message: "",
            count: 0,
            errors: 0,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
/// A structure for constructing a new `Stat` from a set of input arguments
pub struct StatArg {
    pub component: &'static str,
    pub name: &'static str,
    pub value: f64,
    pub account_key: &'static str,
    pub graph_name: &'static str,
}

impl StatArg {
    /// Create a new `Stat`
    pub(crate) fn to_stat(&self) -> Stat {
        Stat {
            component: self.component,
            name: self.name,
            value: self.value,
            account: self.account_key,
            graph: self.graph_name,
            timestamp: Utc::now().timestamp_millis(),
            kind: "",
            count: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// A structure for constructing a new `Logger` from a set of input arguments
pub struct LoggerArg {
    pub graph_name: &'static str,
    pub account_key: &'static str,
    pub is_dev_env: bool,
    pub is_disabled: bool,
}
