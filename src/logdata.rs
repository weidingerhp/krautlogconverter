use ::chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use serde_with::*;

// use chrono::serde::ts_seconds;

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct LogLine {
    #[serde_as(as = "TimestampMilliSeconds")]
    // #[serde(with = "serde_with::chrono::datetime_utc_ts_seconds_from_any")]
    pub time: DateTime<Utc>,
    pub level: u16,
    pub pid: u32,
    pub hostname: String,
    pub msg: Option<String>,

}