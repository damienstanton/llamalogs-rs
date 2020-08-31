use llamalogs::*;

fn foo(mut global: &mut GlobalState) {
    let mut args1 = LogArgs::default();
    args1.sender = "User A";
    args1.receiver = "Web Server";
    args1.message = "Hello from Rust!";
    log(&mut global, args1);
}

fn bar(mut global: &mut GlobalState) {
    let mut args2 = LogArgs::default();
    args2.sender = "User B";
    args2.receiver = "Web Server";
    args2.message = "Hello from Rust client B!";
    log(&mut global, args2);
}

fn main() {
    let mut global = init(InitArgs {
        account_key: "Q8TK-Qh1ykUNGpiL",
        graph_name: "Ferris",
        is_dev_env: false,
        is_disabled: false,
    });

    foo(&mut global);
    bar(&mut global);

    start_timer(&mut global)
}
