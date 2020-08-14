use crate::{log::Log, statistics::Stat};
use chrono::Utc;

#[derive(Default)]
pub struct InitArgs {
    pub account_key: String,
    pub graph_name: String,
    pub is_dev_env: bool,
    pub is_disabled: bool,
}

pub struct LogArgs {
    pub sender: String,
    pub receiver: String,
    pub message: String,
    pub is_error: bool,
    pub account_key: String,
    pub graph_name: String,
}

impl LogArgs {
    pub fn to_log(&self) -> Log {
        let mut l = Log::default();
        l.sender = self.sender.clone();
        l.receiver = self.receiver.clone();
        l.timestamp = Utc::now().timestamp_nanos() / 1e6 as i64;
        l.message = self.message.clone();
        l.is_initial_message = true;
        l.account = self.account_key.clone();
        l.graph = self.graph_name.clone();
        l.is_error = self.is_error;
        l
    }
}

pub struct StatArgs {
    pub component: String,
    pub name: String,
    pub value: f64,
    pub account_key: String,
    pub graph_name: String,
}

impl StatArgs {
    fn to_stat(&self) -> Stat {
        let mut s = Stat::default();
        s.component = self.component.clone();
        s.name = self.component.clone();
        s.value = self.value;
        s.account = self.account_key.clone();
        s.graph = self.graph_name.clone();
        s.timestamp = Utc::now().timestamp_nanos() / 1e6 as i64;
        s
    }
}
