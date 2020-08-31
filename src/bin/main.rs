use llamalogs::*;
fn main() {
    let mut global = init(InitArgs {
        account_key: "Q8TK-Qh1ykUNGpiL",
        graph_name: "Ferris",
        is_dev_env: false,
        is_disabled: false,
    });

    let mut args1 = LogArgs::default();
    args1.sender = "User A";
    args1.receiver = "Web Server";
    args1.message = "Hello from Rust!";

    let mut args2 = LogArgs::default();
    args2.sender = "User B";
    args2.receiver = "Web Server";
    args2.message = "Hello from Rust client B!";

    start_timer(&mut global);
    log(&mut global, args1);
    log(&mut global, args2);
    force_send(&mut global);
}
