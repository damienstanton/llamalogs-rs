use crate::types::{
    AggregateLog, LlamaError, Log, LogArgs, LogData, LogRequest, Stat, StatArgs, StatData,
};
use futures::executor::block_on;
use serde_json;
use std::{collections::HashMap, sync::mpsc::channel, thread, time::Duration};
use surf::Exception;

#[derive(Debug, Clone, Default)]
pub struct Logger {
    pub is_dev_env: bool,
    pub is_disabled: bool,
    pub account_key: &'static str,
    pub graph_name: &'static str,
    pub aggregated_logs: LogData,
    pub aggregated_stats: StatData,
    pub timer_started: bool,
}

impl Logger {
    pub fn start_timer(&mut self) {
        let (tx, rx) = channel();
        let tx1 = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(5));
            tx1.send(true).unwrap();
        });

        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(59_500));
            tx.send(true).unwrap();
        });

        for _ in rx {
            let _ = match self.send_blocking() {
                Ok(_) => (),
                Err(_) => (),
            };
        }
    }

    pub(crate) fn add_stat(&mut self, stat: Stat) {
        match stat.kind {
            "point" => {}
            "average" => self.add_stat_avg(stat),
            "max" => self.add_stat_max(stat),
            _ => return,
        };
    }

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

    pub(crate) fn add_log(&mut self, log: Log) {
        let sender = log.sender;
        let receiver = log.receiver;

        if !self.aggregated_logs.contains_key(sender) {
            self.aggregated_logs.insert(sender, HashMap::new());
        }

        let txmap = self.aggregated_logs.get(sender).unwrap();
        if txmap.get(receiver).is_none() {
            let mut new_rxmap = txmap.clone();
            let mut agg = log.to_aggregate_log();
            if log.is_error {
                agg.errors += 1;
            }
            agg.count += 1;
            if agg.message == "" && !log.is_error {
                agg.message = log.message;
            }
            if agg.error_message == "" && log.is_error {
                agg.error_message = log.message;
            }
            new_rxmap.insert(receiver, agg);
            self.aggregated_logs.insert(sender, new_rxmap);
        } else {
            let mut existing = *self
                .aggregated_logs
                .get(sender)
                .unwrap()
                .get(receiver)
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

    pub(crate) fn send_blocking(&mut self) -> Result<(), LlamaError> {
        match block_on(self.send()) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Log submission error: {:#?}", e.to_string());
                Err(LlamaError::NetError())
            }
        }
    }

    pub fn collect_messages(&mut self) -> (Vec<AggregateLog>, Vec<Stat>) {
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

    pub(crate) async fn send(&mut self) -> Result<(), Exception> {
        let (log_list, stat_list) = self.collect_messages();
        if self.is_dev_env {
            println!("Log list: {:#?}", self.aggregated_logs);
        }
        if self.aggregated_logs.is_empty() && self.aggregated_stats.is_empty() {
            ()
        }

        let mut new_req = LogRequest::default();
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

    pub fn process_log(&mut self, mut log: Log) {
        if log.account == "" {
            log.account = self.account_key;
        }
        if log.graph == "" {
            log.graph = self.graph_name;
        }
        if log.sender == "" || log.receiver == "" || log.account == "" || log.graph == "" {
            return;
        }
        self.add_log(log);
    }

    pub fn process_stat(&mut self, mut stat: Stat) {
        if stat.account == "" {
            stat.account = self.account_key;
        }
        if stat.graph == "" {
            stat.graph = self.graph_name;
        }
        self.add_stat(stat);
    }

    /// Creates a new log for processing
    pub fn log(&mut self, args: LogArgs) {
        if self.is_disabled {
            return;
        }
        let log = args.to_log();
        self.process_log(log);
    }

    /// Creates a new point stat for processing
    pub fn point_stat(&mut self, args: StatArgs) {
        if self.is_disabled {
            return;
        }
        let mut stat = args.to_stat();
        stat.kind = "point";
        self.process_stat(stat);
    }

    /// Creates a new average stat for processing
    pub fn average_stat(&mut self, args: StatArgs) {
        if self.is_disabled {
            return;
        }
        let mut stat = args.to_stat();
        stat.kind = "average";
        self.process_stat(stat);
    }

    /// Creates a new max stat for processing
    pub fn max_stat(&mut self, args: StatArgs) {
        if self.is_disabled {
            return;
        }
        let mut stat = args.to_stat();
        stat.kind = "max";
        self.process_stat(stat);
    }

    /// Calls a blocking send of the current collection of logs and stats
    pub fn force_send(&mut self) -> Result<(), LlamaError> {
        if self.is_disabled {
            ()
        }
        match self.send_blocking() {
            Ok(_) => Ok(()),
            Err(_) => Err(LlamaError::NetError()),
        }
    }
}
