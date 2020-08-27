use crate::types::{Log, Stat};
use serde::Serialize;
use std::{collections::HashMap, sync::RwLock};
#[derive(Serialize)]
pub(crate) struct Payload {
    pub(crate) account_key: String,
    pub(crate) time_logs: Vec<Log>,
    pub(crate) time_stats: Vec<Stat>,
}

pub(crate) type GlobalState = RwLock<State>;
pub(crate) struct State {
    pub(crate) is_dev_env: bool,
    pub(crate) is_disabled: bool,
    pub(crate) account_key: String,
    pub(crate) graph_name: String,
    pub(crate) aggregated_logs: HashMap<String, HashMap<String, Log>>,
    pub(crate) aggregated_stats: HashMap<String, HashMap<String, Stat>>,
}
