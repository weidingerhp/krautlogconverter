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

#[cfg(test)]
#[test]
pub fn test_simple() {
    let line:LogLine = serde_json::from_str(r#"{"level":30,"time":1642090836068,"pid":10,"hostname":"a94a64688df8","msg":"App was instantiated"}"#).unwrap();
    assert_eq!(line.time.to_string(), "2022-01-13 16:20:36.068 UTC");
    assert_eq!(line.pid, 10);
    assert_eq!(line.hostname, "a94a64688df8");
    assert_eq!(line.msg.unwrap(), "App was instantiated");

}