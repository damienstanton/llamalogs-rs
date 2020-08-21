//! A Rust client for [llamalogs][1].
//! [1]: https://llamalogs.com/
//! Llama Logs is a brand new tool that turns distributed logs into a real time
//! interactive graph. It was created to help bring clarity to complex cloud
//! architectures
use async_std::io::Error;
use std::collections::HashMap;

struct Llama {
    is_dev_env: bool,
    is_disabled: bool,
    global_account_key: &'static str,
    global_graph_name: &'static str,
}

enum LogType {
    Send,
    Return,
}

struct Log<'a> {
    log_type: LogType,
    sender: HashMap<&'static str, &'a Log<'a>>,
    receiver: HashMap<&'static str, &'a Log<'a>>,
    timestamp: i64,
    message: &'static str,
    initial_message: bool,
    account: &'static str,
    graph: &'static str,
    is_error: bool,
    elapsed: i64,
    account_key: &'static str,
    graph_name: &'static str,
    start_time: i64,
}

struct Stat {
    component: &'static str,
    name: &'static str,
    value: i64,
    stat_type: &'static str,
    timestamp: i64,
    account: &'static str,
    graph: &'static str,
    count: Option<i64>,
}

struct Aggregator<'a> {
    timeout_clear: Option<bool>,
    last_send_time: i64,
    aggregate_logs: Box<Vec<Log<'a>>>,
    aggregate_stats: Box<Vec<Stat>>,
}

impl<'a> Aggregator<'a> {
    fn start_sending(&mut self, global: &Llama) {
        self.set_new_timeout(global);
    }

    async fn add_time(&mut self, global: &Llama) {
        if self.timeout_clear.is_some() {
            let now = 0; // TODO: chrono this to utc now
            let mut poll_time = 54500;
            if global.is_dev_env {
                poll_time = 5000;
                if self.last_send_time < now - poll_time {
                    return;
                }
                self.clear_time(global).await;
                self.set_new_timeout(global).await;
            } else {
                self.set_new_timeout(global).await;
            }
        }
    }

    async fn clear_time(&mut self, global: &Llama) {
        if self.timeout_clear.is_none() {
            self.timeout_clear = None;
            self.send_messages(global).await;
        }
    }

    async fn set_new_timeout(&mut self, global: &Llama) {
        self.timeout_clear = None;
        std::thread::sleep(std::time::Duration::from_millis(5000));
        self.send_messages(global).await;
    }

    async fn send_messages(&mut self, global: &Llama) {
        let (log_list, stat_list) = match self.collect_messages(global).await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("log and stat collection error: {:?}", e.to_string());
                (Vec::new(), Vec::new())
            }
        };
        proxy_messages(&log_list, &stat_list);
        self.last_send_time = 0 // chrono this to utc now
    }

    async fn collect_messages(
        &'a self,
        global: &Llama,
    ) -> Result<(Vec<Log<'a>>, Vec<Stat>), Error> {
        if global.is_dev_env {
            println!("sending messages");
        }

        Ok((Vec::new(), Vec::new()))
    }
}

fn proxy_messages(log_list: &Vec<Log>, stat_list: &Vec<Stat>) {
    // TODO
}
