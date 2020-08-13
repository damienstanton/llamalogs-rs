//! Core logging data structures
use serde::{Deserialize, Serialize};
use serde_json::to_string as to_json_string;
use thiserror::Error;
#[derive(Default, Debug)]
pub struct Log {
    pub sender: String,
    pub receiver: String,
    pub timestamp: i64,
    pub message: String,
    pub is_initial_message: bool,
    pub account: String,
    pub graph: String,
    pub is_error: bool,
    pub elapsed: isize,
}

impl Log {
    async fn to_aggregate(&self) -> AggregatedLog {
        let mut al = AggregatedLog::default();
        al.sender = self.sender.clone();
        al.receiver = self.receiver.clone();
        al.account = self.account.clone();
        al.graph = self.graph.clone();
        al
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AggregatedLog {
    pub sender: String,
    pub receiver: String,
    pub account: String,
    pub graph: String,
    pub count: isize,
    pub errors: isize,
    pub elapsed: isize,
    pub message: String,
    pub error_message: String,
    pub initial_message_count: isize,
}

#[derive(Error, Debug)]
pub(crate) enum JSONFailure {
    #[error("JSON conversion failed")]
    ConversionError,
    #[error("An unknown error occurred")]
    UnknownError,
}

impl AggregatedLog {
    async fn to_json(&self) -> Result<String, JSONFailure> {
        match to_json_string(self) {
            Ok(s) => Ok(s),
            Err(_) => Err(JSONFailure::ConversionError),
        }
    }
}
