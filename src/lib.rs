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

const POLL_SHORT: u64 = 5;
const POLL_LONG: u64 = 5000; // TODO: 59_500

/// The public logger instance
#[derive(Debug)]
pub struct Logger {
    state: InnerLogger,
    request: Request,
    tx: Arc<Mutex<mpsc::Sender<Arc<InnerLogger>>>>,
    rx: Arc<Mutex<mpsc::Receiver<Arc<InnerLogger>>>>,
}

impl Logger {
    fn start_timer(mut self) -> Self {
        self
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
        logger.start_timer()
    }

    /// Create a new LLamalogs log and add it to the queue
    pub fn log(&mut self, args: LogArg) {
        if self.state.is_disabled {
            return;
        }
        self.state.add_log(args.to_log());
    }

    /// Create a new Llamalogs stat and add it to the queue
    pub fn stat(&mut self, args: StatArg) {
        if self.state.is_disabled {
            return;
        }
        self.state.add_stat(args.to_stat());
    }

    /// Force send the current queue of logs and stats
    pub fn force_send(&mut self) -> Result<(), &'static str> {
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
