use crate::types::*;
use futures::executor::block_on;
use serde_json::to_string;
use surf::Exception;

pub(crate) fn send_blocking(global: &GlobalState) -> Result<(), LlamaError> {
    match block_on(send(global)) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Log submission error: {:#?}", e.to_string());
            Err(LlamaError::NetError())
        }
    }
}

pub fn collect_messages(global: &GlobalState) -> (Vec<AggregateLog>, Vec<Stat>) {
    let mut log_list = Vec::new();
    let mut stat_list = Vec::new();

    for (_, k) in &global.aggregated_logs {
        for (_, v) in k {
            log_list.push(*v);
        }
    }
    for (_, k) in &global.aggregated_stats {
        for (_, v) in k {
            stat_list.push(*v);
        }
    }

    (log_list, stat_list)
}

pub(crate) async fn send(global: &GlobalState) -> Result<(), Exception> {
    let (log_list, stat_list) = collect_messages(global);
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
    let new_req_json = to_string(&new_req).unwrap();
    println!("JSON request: {:#?}", new_req_json);

    let url = match global.is_dev_env {
        true => "http://localhost:4000/api/v0/timedata",
        false => "https://llamalogs.com/api/v0/timedata",
    };

    let mut res = surf::post(url)
        .body_json(&new_req).unwrap().await.unwrap();

    println!("Status:\t{} \nInfo:\t{}\n", 
        res.status(), 
        res.body_string().await.unwrap());

    Ok(())
}
