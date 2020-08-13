use serde::{Deserialize, Serialize};
#[derive(Default, Deserialize, Serialize)]
pub struct Stat {
    pub component: String,
    pub name: String,
    pub kind: String,
    pub value: f64,
    pub account: String,
    pub graph: String,
    pub timestamp: i64,
    pub count: i64,
}
