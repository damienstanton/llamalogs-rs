use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) enum LogType {
    Send,
    Return,
}

#[derive(Debug, Serialize)]
pub(crate) struct Log {
    // TODO check go struct tags for serde renames
    // #[serde(rename(serialize = "ser_name"))]
    pub(crate) log_type: LogType,
    pub(crate) sender: String,
    pub(crate) receiver: String,
    pub(crate) timestamp: i64,
    pub(crate) message: String,
    pub(crate) initial_message: bool,
    pub(crate) account: String,
    pub(crate) graph: String,
    pub(crate) is_error: bool,
    pub(crate) elapsed: i64,
    pub(crate) account_key: String,
    pub(crate) graph_name: String,
    pub(crate) start_time: i64,
    pub(crate) initial_message_count: i64,
    pub(crate) error_message: String,
    pub(crate) count: i64,
    pub(crate) errors: i64,
}

#[derive(Serialize)]
pub(crate) struct Stat {
    pub(crate) component: String,
    pub(crate) name: String,
    pub(crate) value: i64,
    pub(crate) stat_type: String,
    pub(crate) timestamp: i64,
    pub(crate) account: String,
    pub(crate) graph: String,
    pub(crate) count: Option<i64>,
}
