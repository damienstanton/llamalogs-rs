use crate::types::*;
use std::collections::HashMap;

pub(crate) fn start_timer(mut global: GlobalState) {
    if global.timer_started {
        return;
    }

    // TODO: sleep for 5 seconds then send_messages on background thread
    // TODO: set up ticker for 59_500 millis, send_messages every tick

    global.timer_started = true;
}

pub(crate) fn get_and_clear_logs(mut global: GlobalState) -> (LogData, StatData) {
    let current_logs = global.aggregated_logs;
    let current_stats = global.aggregated_stats;
    global.aggregated_logs = HashMap::new();
    global.aggregated_stats = HashMap::new();
    (current_logs, current_stats)
}

pub(crate) fn add_stat(mut global: GlobalState, stat: Stat) {
    match stat.kind {
        "point" => {}
        "average" => add_stat_avg(global, stat),
        "max" => add_stat_max(global, stat),
        _ => return,
    };
}

pub(crate) fn add_stat_avg(mut global: GlobalState, mut stat: Stat) {
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

pub(crate) fn add_stat_max(mut global: GlobalState, stat: Stat) {
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

pub(crate) fn add_log(mut global: GlobalState, log: Log) {
    let sender = log.sender;
    let receiver = log.receiver;

    if !global.aggregated_logs.contains_key(sender) {
        global.aggregated_logs.insert(sender, HashMap::new());
    }
    let txmap = global.aggregated_logs.get(sender).unwrap();
    if txmap.get(receiver).is_none() {
        let agg = log.to_aggregate_log();
        let mut new_rxmap = HashMap::new();
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

    existing.count += 1;
    if existing.log.message == "" && !log.is_error {
        existing.log.message = log.message;
    }
    if existing.error_message == "" && log.is_error {
        existing.error_message = log.message;
    }

    if global.is_dev_env {
        println!("{:#?}", global.aggregated_logs);
    }
}
