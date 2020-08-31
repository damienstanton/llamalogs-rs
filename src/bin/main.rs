use llamalogs::*;

fn foo(logger: &mut Logger) {
    let mut args1 = LogArgs::default();
    args1.sender = "User A";
    args1.receiver = "Web Server";
    args1.message = "Hello from Rust!";
    logger.log(args1);
}

fn bar(logger: &mut Logger) {
    let mut args2 = LogArgs::default();
    args2.sender = "User B";
    args2.receiver = "Web Server";
    args2.message = "Hello from Rust client B!";
    logger.log(args2);
}

fn main() {
    let mut logger = init(InitArgs {
        account_key: "Q8TK-Qh1ykUNGpiL",
        graph_name: "Ferris",
        is_dev_env: false,
        is_disabled: false,
    });

    foo(&mut logger);
    bar(&mut logger);
    logger.start_timer();
}
