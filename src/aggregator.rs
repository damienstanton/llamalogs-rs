use crate::{
    state::{GlobalState, Payload},
    types::{Log, Stat},
};
use chrono::Utc;
use std::{collections::HashMap, sync::Arc};
use surf::Exception;
pub(crate) struct Aggregator {
    pub(crate) timeout_clear: Option<bool>,
    pub(crate) last_send_time: i64,
}

impl Aggregator {
    pub(crate) fn start_sending(&mut self, global: &GlobalState) {
        self.set_new_timeout(global);
    }

    pub(crate) async fn add_time(&mut self, global: &GlobalState) {
        if self.timeout_clear.is_some() {
            let now = Utc::now().timestamp();
            let mut poll_time = 54500;
            if global.read().unwrap().is_dev_env {
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

    pub(crate) async fn clear_time(&mut self, global: &GlobalState) {
        if self.timeout_clear.is_none() {
            self.timeout_clear = None;
            self.send_messages(global).await;
        }
    }

    pub(crate) async fn set_new_timeout(&mut self, global: &GlobalState) {
        self.timeout_clear = None;
        std::thread::sleep(std::time::Duration::from_millis(5000));
        self.send_messages(global).await;
    }

    pub(crate) async fn send_messages(&mut self, global: &GlobalState) {
        let (log_list, stat_list) = match self.collect_messages(global).await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("log and stat collection error: {:?}", e.to_string());
                (Vec::new(), Vec::new())
            }
        };
        self.proxy_messages(log_list, stat_list, global).await;
        self.last_send_time = Utc::now().timestamp()
    }

    pub(crate) async fn collect_messages(
        &self,
        global: &GlobalState,
    ) -> Result<(Vec<Log>, Vec<Stat>), Exception> {
        if global.read().unwrap().is_dev_env {
            println!("sending messages");
        }
        let mut logs = Vec::<Log>::new();
        let mut stats = Vec::<Stat>::new();

        // TODO: loop through and construct logs/stats
        global
            .read()
            .unwrap()
            .aggregated_logs
            .keys()
            .for_each(|sender| {
                let rx = global
                    .read()
                    .unwrap()
                    .aggregated_logs
                    .get(sender)
                    .unwrap()
                    .keys()
                    .for_each(|receiver| {
                        // logs.push(
                        //     Log {
                        //         sender,
                        //         receiver,
                        //         count: global.read()
                        //             .unwrap()
                        //             .aggregated_logs
                        //             .get(sender)
                        //             .unwrap()
                        //             .get(receiver)
                        //             .unwrap().count,
                        //         errors: global.read()
                        //             .unwrap()
                        //             .aggregated_logs
                        //             .get(sender)
                        //             .unwrap()
                        //             .get(receiver)
                        //             .unwrap().errors,
                        //         message: global.read()
                        //             .unwrap()
                        //             .aggregated_logs
                        //             .get(sender)
                        //             .unwrap()
                        //             .get(receiver)
                        //             .unwrap().message,
                        //         error_message: global.read()
                        //             .unwrap()
                        //             .aggregated_logs
                        //             .get(sender)
                        //             .unwrap()
                        //             .get(receiver)
                        //             .unwrap().error_message,
                        //         graph: global.read()
                        //             .unwrap()
                        //             .aggregated_logs
                        //             .get(sender)
                        //             .unwrap()
                        //             .get(receiver)
                        //             .unwrap_or("noGraph").graph,
                        //         account: global.read()
                        //             .unwrap()
                        //             .aggregated_logs
                        //             .get(sender)
                        //             .unwrap()
                        //             .get(receiver)
                        //             .unwrap().account,
                        //         initial_message_count: global.read()
                        //             .unwrap()
                        //             .aggregated_logs
                        //             .get(sender)
                        //             .unwrap()
                        //             .get(receiver)
                        //             .unwrap().initial_message_count
                        //     }
                        // );
                    });
            });

        global.write().unwrap().aggregated_logs = HashMap::new();
        global.write().unwrap().aggregated_stats = HashMap::new();
        Ok((logs, stats))
    }

    async fn proxy_messages(
        &self,
        time_logs: Vec<Log>,
        time_stats: Vec<Stat>,
        global: &GlobalState,
    ) -> Result<(), Exception> {
        if !time_logs.is_empty() || !time_stats.is_empty() {
            let mut account_key = String::new();
            if !time_logs.is_empty() {
                account_key = time_logs[0].account.to_owned();
            }
            if !time_stats.is_empty() {
                account_key = time_stats[0].account.to_owned();
            }

            let url = match global.read().unwrap().is_dev_env {
                true => "http://localhost:4000/",
                false => "https://llamalogs.com/",
            };

            if global.read().unwrap().is_dev_env {
                println!("Log list: {:#?}", time_logs);
            }

            let res = surf::post(url)
                .body_json(&Payload {
                    account_key,
                    time_logs,
                    time_stats,
                })?
                .await?;
        }

        Ok(())
    }
}
