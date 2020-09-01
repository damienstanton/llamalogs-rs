use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surf::Exception;

#[derive(Debug, Serialize, Clone, Copy, Deserialize)]
/// A log structure
pub struct Log {
    pub account_key: &'static str,
    pub graph: &'static str,
    pub start_time: i64,
    #[serde(rename(serialize = "initialMessageCount"))]
    pub initial_message_count: i64,
    #[serde(rename(serialize = "errorMessage"))]
    pub error_message: &'static str,
    pub count: i64,
    pub errors: i64,
    pub message: &'static str,
    pub sender: &'static str,
    pub receiver: &'static str,
    pub is_error: bool,
    pub timestamp: i64,
    pub is_initial_message: bool,
    pub elapsed: i64,
}

#[derive(Debug, Serialize, Clone, Copy, Deserialize)]
/// A statistic structure
pub struct Stat {
    pub component: &'static str,
    pub name: &'static str,
    pub value: f64,
    pub kind: &'static str,
    pub timestamp: i64,
    pub account: &'static str,
    pub graph: &'static str,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
/// A structure for sending a network request to the Llamalogs service
pub(crate) struct Request {
    pub(crate) account_key: &'static str,
    pub(crate) time_logs: Vec<Log>,
    pub(crate) time_stats: Vec<Stat>,
}

#[derive(Debug, Clone, Default)]
/// The central data sructure for the logger instance
pub struct Logger {
    pub is_dev_env: bool,
    pub is_disabled: bool,
    pub account_key: &'static str,
    pub graph_name: &'static str,
    pub aggregated_logs: HashMap<&'static str, HashMap<&'static str, Log>>,
    pub aggregated_stats: HashMap<&'static str, HashMap<&'static str, Stat>>,
    pub timer_started: bool,
}

impl Logger {
    /// Add a new log for processing
    pub fn add_log(&mut self, mut log: Log) {
        if log.account_key == "" {
            log.account_key = self.account_key;
        }
        if log.graph == "" {
            log.graph = self.graph_name;
        }
        if log.sender == "" || log.receiver == "" || log.account_key == "" || log.graph == "" {
            return;
        }

        if !self.aggregated_logs.contains_key(&log.sender) {
            self.aggregated_logs.insert(log.sender, HashMap::new());
        }

        let txmap = self.aggregated_logs.get(&log.sender).unwrap();
        if txmap.get(&log.receiver).is_none() {
            let mut new_rxmap = txmap.clone();
            if log.is_error {
                log.errors += 1;
            }
            log.count += 1;
            if log.error_message == "" && log.is_error {
                log.error_message = log.message;
            }
            new_rxmap.insert(log.receiver, log);
            self.aggregated_logs.insert(log.sender, new_rxmap);
        } else {
            let mut existing = *self
                .aggregated_logs
                .get(&log.sender)
                .unwrap()
                .get(&log.receiver)
                .unwrap();
            if log.is_error {
                existing.errors += 1;
            }
            existing.count += 1;
            if existing.message == "" && !log.is_error {
                existing.message = log.message;
            }
            if existing.error_message == "" && log.is_error {
                existing.error_message = log.message;
            }
        }

        if self.is_dev_env {
            println!("Aggregated logs: {:#?}", self.aggregated_logs);
        }
    }

    /// Add a single statistic
    pub fn add_stat(&mut self, mut stat: Stat) {
        if stat.account == "" {
            stat.account = self.account_key;
        }
        if stat.graph == "" {
            stat.graph = self.graph_name;
        }
        match stat.kind {
            "point" => {}
            "average" => self.add_stat_avg(stat),
            "max" => self.add_stat_max(stat),
            _ => return,
        };
    }

    /// Extract vectors of all aggregated logs and statistics, and clear their
    /// entries in the global `Logger` structure.
    pub(crate) fn collect_and_clear(&mut self) -> (Vec<Log>, Vec<Stat>) {
        let mut log_list = Vec::new();
        let mut stat_list = Vec::new();

        for (_, k) in &self.aggregated_logs {
            for (_, v) in k {
                log_list.push(*v);
            }
        }
        for (_, k) in &self.aggregated_stats {
            for (_, v) in k {
                stat_list.push(*v);
            }
        }

        self.aggregated_logs = HashMap::new();
        self.aggregated_stats = HashMap::new();

        (log_list, stat_list)
    }

    /// Add an average statistic
    pub(crate) fn add_stat_avg(&mut self, mut stat: Stat) {
        let component = stat.component;
        let name = stat.name;
        if self.aggregated_stats.get(component).is_none() {
            self.aggregated_stats.insert(component, HashMap::new());
        }
        if self
            .aggregated_stats
            .get(component)
            .unwrap()
            .get(name)
            .is_none()
        {
            let mut new_named_stat = HashMap::new();
            new_named_stat.insert(component, stat);
            self.aggregated_stats.insert(name, new_named_stat);
            stat.count = 0;
        }

        let mut existing = *self
            .aggregated_stats
            .get(component)
            .unwrap()
            .get(name)
            .unwrap();

        existing.value += stat.value;
        existing.count += 1;
    }

    /// Add an max statistic
    pub(crate) fn add_stat_max(&mut self, stat: Stat) {
        let component = stat.component;
        let name = stat.name;
        if self.aggregated_stats.get(component).is_none() {
            self.aggregated_stats.insert(component, HashMap::new());
        }
        if self
            .aggregated_stats
            .get(component)
            .unwrap()
            .get(name)
            .is_none()
        {
            let mut new_named_stat = HashMap::new();
            new_named_stat.insert(component, stat);
            self.aggregated_stats.insert(name, new_named_stat);
        } else {
            let mut existing = *self
                .aggregated_stats
                .get(component)
                .unwrap()
                .get(name)
                .unwrap();

            if stat.value > existing.value {
                existing.value = stat.value;
            }
        }
    }

    /// A blocking send to the network
    pub(crate) fn send_blocking(&mut self) -> Result<(), &'static str> {
        match block_on(self.send()) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Log submission error: {:#?}", e.to_string());
                Err("network error")
            }
        }
    }

    /// An asynchronouse send to the network
    pub(crate) async fn send(&mut self) -> Result<(), Exception> {
        let (log_list, stat_list) = self.collect_and_clear();
        if self.is_dev_env {
            println!("Log list: {:#?}", self.aggregated_logs);
        }
        if self.aggregated_logs.is_empty() && self.aggregated_stats.is_empty() {
            ()
        }

        let mut new_req = Request::default();
        if !log_list.is_empty() {
            new_req.account_key = log_list[0].account_key
        }
        if !stat_list.is_empty() {
            new_req.account_key = stat_list[0].account
        }

        new_req.time_logs = log_list;
        new_req.time_stats = stat_list;
        let new_req_json = serde_json::to_string(&new_req).unwrap();
        println!("JSON request: {:#?}", new_req_json);

        let url = match self.is_dev_env {
            true => "http://localhost:4000/api/v0/timedata",
            false => "https://llamalogs.com/api/v0/timedata",
        };

        let mut res = surf::post(url).body_json(&new_req).unwrap().await.unwrap();

        println!(
            "Status:\t{} \nInfo:\t{}\n",
            res.status(),
            res.body_string().await.unwrap()
        );

        Ok(())
    }
}
