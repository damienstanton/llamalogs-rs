//! A Rust client for [llamalogs][1].
//! [1]: https://llamalogs.com/
//! Llama Logs is a brand new tool that turns distributed logs into a real time
//! interactive graph. It was created to help bring clarity to complex cloud
//! architectures
mod args;
mod types;

pub use args::{LogArg, LoggerArg, StatArg};
use std::sync::{mpsc, Arc, Mutex};
use types::{Logger as InnerLogger, Request};

/// The public logger instance
#[derive(Debug)]
pub struct Logger {
    state: InnerLogger,
    request: Request,
    tx: Arc<Mutex<mpsc::Sender<InnerLogger>>>,
    rx: Arc<Mutex<mpsc::Receiver<InnerLogger>>>,
}

impl Logger {
    pub fn send_every(mut self, millis: u64) {
        std::thread::spawn(move || loop {
            println!("Polling for {} milliseconds...", millis);
            std::thread::sleep(std::time::Duration::from_millis(millis));
            if let Ok(guard) = self.rx.try_lock() {
                while let Ok(updated_state) = guard.try_recv() {
                    self.state = updated_state;
                    self.force_send().unwrap();
                    self.state.clear();
                }
            }
        })
        .join()
        .unwrap();
    }

    /// Create a new Llamalogs logger from an `LoggerArgs` structure
    pub fn from_args(args: LoggerArg) -> Self {
        let mut state = InnerLogger::default();
        state.account_key = args.account_key;
        state.graph_name = args.graph_name;
        state.is_disabled = args.is_disabled;
        state.is_dev_env = args.is_dev_env;
        let (tx, rx) = mpsc::channel();
        let logger = Self {
            state,
            request: Request::default(),
            tx: Arc::new(Mutex::new(tx)),
            rx: Arc::new(Mutex::new(rx)),
        };
        logger
    }

    /// Create a new LLamalogs log and add it to the queue
    pub fn log(&mut self, args: LogArg) {
        if self.state.is_disabled {
            return;
        }
        let mut new_state = self.state.clone();
        let log_data = args.to_log();

        if self.state.aggregated_logs.get(log_data.sender).is_some() {
            let m = self.state.aggregated_logs.get(log_data.sender).unwrap();
            if let Some(&log) = m.get(log_data.receiver) {
                if log == log_data {
                    return;
                }
            }
        }
        new_state.add_log(log_data);
        if let Ok(chan) = self.tx.try_lock() {
            chan.send(new_state).unwrap();
        }
    }

    /// Create a new Llamalogs stat and add it to the queue
    pub fn stat(&mut self, args: StatArg) {
        if self.state.is_disabled {
            return;
        }
        let mut new_state = self.state.clone();
        new_state.add_stat(args.to_stat());
        if let Ok(chan) = self.tx.try_lock() {
            chan.send(new_state).unwrap();
        }
    }

    /// Force send the current queue of logs and stats
    pub fn force_send(&self) -> Result<(), &'static str> {
        if self.state.is_disabled {
            ()
        }
        match self.state.send_blocking() {
            Ok(_) => Ok(()),
            Err(_) => Err("network error"),
        }
    }

    /* TODO: Potentially add these in for direct parameterization? */
    // /// Create a new Llamalogs stat and add it to the queue
    // pub fn stat(&mut self, component: &'static str, name: &'static str, value: f64) {}

    // /// Create a new LLamalogs log and add it to the queue
    // pub fn log(
    //     &mut self,
    //     sender: &'static str,
    //     receiver: &'static str,
    //     message: &'static str,
    //     is_error: bool,
    // ) {
    // }
}
