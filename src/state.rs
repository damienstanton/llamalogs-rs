use crate::{log::AggregatedLog, statistics::Stat};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub type GlobalState = Arc<Mutex<Global>>;

type LogMap = HashMap<String, HashMap<String, AggregatedLog>>;
type StatMap = HashMap<String, HashMap<String, Stat>>;

#[derive(Default)]
pub struct Global {
    pub aggregate_logs: Box<LogMap>,
    pub aggregate_stats: Box<StatMap>,
    pub timer_started: bool,
    pub graph_name: String,
    pub account_key: String,
    pub is_dev_env: bool,
    pub is_disabled: bool,
}

impl Global {
    pub fn new() -> GlobalState {
        Arc::new(Mutex::new(Global::default()))
    }
}
