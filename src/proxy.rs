use crate::types::*;
use futures::executor::block_on;
use surf::Exception;

pub(crate) fn collect_and_send_blocking(global: GlobalState) -> Result<(), Exception> {
    block_on(collect_and_send(global))?;
    Ok(())
}

pub(crate) async fn collect_and_send(global: GlobalState) -> Result<(), Exception> {
    let log_list = global
        .aggregated_logs
        .iter()
        .flat_map(|log| log.1.values())
        .map(|ag| *ag)
        .collect::<Vec<AggregateLog>>();

    let stat_list = global
        .aggregated_stats
        .iter()
        .flat_map(|log| log.1.values())
        .map(|ag| *ag)
        .collect::<Vec<Stat>>();

    if global.is_dev_env {
        println!("Log list: {:#?}", global.aggregated_logs);
    }
    if global.aggregated_logs.is_empty() && global.aggregated_stats.is_empty() {
        ()
    }

    let mut new_req = LogRequest::default();
    if !log_list.is_empty() {
        new_req.account_key = log_list[0].account_key
    }
    if !stat_list.is_empty() {
        new_req.account_key = stat_list[0].account
    }
    new_req.time_logs = log_list;
    new_req.time_stats = stat_list;

    let url = match global.is_dev_env {
        true => "http://localhost:4000/",
        false => "https://llamalogs.com/api/v0/timedata",
    };
    let res = surf::post(url).body_json(&new_req)?.await?;
    if !res.status().is_success() {
        eprintln!("Bad status code: {:#?}", res.status());
    }

    Ok(())
}
