use llamalogs::*;
fn main() {
    let mut global = init(InitArgs {
        account_key: "Q8TK-Qh1ykUNGpiL",
        graph_name: "Ferris",
        is_dev_env: false,
        is_disabled: false,
    });

    let mut args = LogArgs::default();
    args.sender = "User";
    args.receiver = "Web Server";
    args.message = "Hello from Rust!";

    log(&mut global, args);
    force_send(global);
}
