use llamalogs::{LogArg, Logger, LoggerArg};

fn main() {
    // Configuration
    let account_key = "Q8TK-Qh1ykUNGpiL";
    let graph_name = "Ferris";
    let args = LoggerArg {
        account_key,
        graph_name,
        is_dev_env: false,
        is_disabled: false,
    };
    let mut logger = Logger::from_args(args);

    // Build some logs
    let log1 = LogArg {
        sender: "User A",
        receiver: "Web Server",
        message: "Hello from Rust!",
        account_key,
        is_error: false,
        graph_name,
    };

    let log2 = LogArg {
        sender: "User B",
        receiver: "Web Server",
        message: "Hello (again) from Rust!",
        account_key,
        is_error: false,
        graph_name,
    };

    std::thread::sleep(std::time::Duration::from_secs(10));
    let log3 = LogArg {
        sender: "User C",
        receiver: "Web Server",
        message: "Hello!",
        account_key,
        is_error: false,
        graph_name,
    };

    logger.log(log1);
    logger.log(log2);
    logger.log(log3);
    println!("{:#?}", logger.force_send().unwrap());
}
