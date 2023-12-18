use chrono::{serde::ts_seconds, DateTime, Utc};
use std::{borrow::Cow, net::IpAddr};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CaptchaSolution<'a, T> {
    /// The task id is not returned by 2captcha, instead it
    /// is manually added to the struct in order to allow the
    /// use of the [crate::CaptchaSolver::report] method without
    /// allowing the user to see (or more importantly, change)
    /// the task id
    #[serde(default = "Default::default")]
    pub(crate) task_id: u64,

    /// The actual solution to the captcha
    pub solution: T,

    /// The task price charged from your balance
    pub cost: Cow<'a, str>,

    /// Timestamp indicating the moment task was submitted
    #[serde(with = "ts_seconds")]
    pub create_time: DateTime<Utc>,

    /// Timestamp indicating the moment task was completed
    #[serde(with = "ts_seconds")]
    pub end_time: DateTime<Utc>,

    /// The number of workers attempted to complete your task
    pub solve_count: u8,

    /// The IP address that submitted the task request
    pub ip: IpAddr,
}
