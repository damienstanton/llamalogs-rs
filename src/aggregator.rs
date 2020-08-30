use crate::proxy::*;
use crate::types::*;
use std::{collections::HashMap, sync::mpsc::channel, thread};

pub(crate) fn start_timer(global: &mut GlobalState) {
    if global.timer_started {
        return;
    }
    global.timer_started = true;
    let fs_global = global.clone();

    thread::spawn(move || {
        thread::sleep(std::time::Duration::from_secs(5));
        let res = send_blocking(&fs_global);
        match res {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Ticker log submission error: {:#?}", e.to_string());
                ()
            }
        }
    });

    let (tx, rx) = channel();
    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_millis(59_500));
        let _ = tx.send(true);
    });
    while let Some(_) = rx.recv().iter().next() {
        match send_blocking(&global) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Timer submission error: {:#?}", e.to_string());
                ()
            }
        };
    }
}

pub(crate) fn add_stat(global: &mut GlobalState, stat: Stat) {
    match stat.kind {
        "point" => {}
        "average" => add_stat_avg(global, stat),
        "max" => add_stat_max(global, stat),
        _ => return,
    };
}

pub(crate) fn add_stat_avg(global: &mut GlobalState, mut stat: Stat) {
    let component = stat.component;
    let name = stat.name;
    if global.aggregated_stats.get(component).is_none() {
        global.aggregated_stats.insert(component, HashMap::new());
    }
    if global
        .aggregated_stats
        .get(component)
        .unwrap()
        .get(name)
        .is_none()
    {
        let mut new_named_stat = HashMap::new();
        new_named_stat.insert(component, stat);
        global.aggregated_stats.insert(name, new_named_stat);
        stat.count = 0;
    }

    let mut existing = *global
        .aggregated_stats
        .get(component)
        .unwrap()
        .get(name)
        .unwrap();

    existing.value += stat.value;
    existing.count += 1;
}

pub(crate) fn add_stat_max(global: &mut GlobalState, stat: Stat) {
    let component = stat.component;
    let name = stat.name;
    if global.aggregated_stats.get(component).is_none() {
        global.aggregated_stats.insert(component, HashMap::new());
    }
    if global
        .aggregated_stats
        .get(component)
        .unwrap()
        .get(name)
        .is_none()
    {
        let mut new_named_stat = HashMap::new();
        new_named_stat.insert(component, stat);
        global.aggregated_stats.insert(name, new_named_stat);
    }

    let mut existing = *global
        .aggregated_stats
        .get(component)
        .unwrap()
        .get(name)
        .unwrap();

    if stat.value > existing.value {
        existing.value = stat.value;
    }
}

pub(crate) fn add_log(global: &mut GlobalState, log: Log) {
    let sender = log.sender;
    let receiver = log.receiver;

    if !global.aggregated_logs.contains_key(sender) {
        global.aggregated_logs.insert(sender, HashMap::new());
    }

    let txmap = global.aggregated_logs.get(sender).unwrap();
    if txmap.get(receiver).is_none() {
        let mut new_rxmap = txmap.clone();
        let agg = log.to_aggregate_log();
        new_rxmap.insert(receiver, agg);
        global.aggregated_logs.insert(sender, new_rxmap);
    }

    let mut existing = *global
        .aggregated_logs
        .get(sender)
        .unwrap()
        .get(receiver)
        .unwrap();
    if log.is_error {
        existing.errors += 1;
    }
    // TODO: This is not updating the actual object in the global store.
    existing.count += 1;
    if existing.message == "" && !log.is_error {
        existing.message = log.message;
    }
    if existing.error_message == "" && log.is_error {
        existing.error_message = log.message;
    }

    if global.is_dev_env {
        println!("Aggregated logs: {:#?}", global.aggregated_logs);
    }
}
