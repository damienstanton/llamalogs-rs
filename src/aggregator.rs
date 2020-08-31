use crate::proxy::*;
use crate::types::*;
use std::{
    collections::HashMap,
    sync::{mpsc::channel, Arc},
    thread,
};

pub fn start_timer(global: &mut GlobalState) {
    // TODO
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
        let mut agg = log.to_aggregate_log();
        if log.is_error {
            agg.errors += 1;
        }
        agg.count += 1;
        if agg.message == "" && !log.is_error {
            agg.message = log.message;
        }
        if agg.error_message == "" && log.is_error {
            agg.error_message = log.message;
        }
        new_rxmap.insert(receiver, agg);
        global.aggregated_logs.insert(sender, new_rxmap);
    } else {
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
        if existing.message == "" && !log.is_error {
            existing.message = log.message;
        }
        if existing.error_message == "" && log.is_error {
            existing.error_message = log.message;
        }
    }

    if global.is_dev_env {
        println!("Aggregated logs: {:#?}", global.aggregated_logs);
    }
}
