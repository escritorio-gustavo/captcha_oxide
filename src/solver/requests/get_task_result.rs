use std::borrow::Cow;

use crate::solution::CaptchaSolution;

// Request

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetTaskResultRequest<'a> {
    pub client_key: &'a str,
    pub task_id: u64,
}

// The response had to be split into two structs because
// I couldn't make serde untagged enum to recognize the
// success case

// Response (error)

#[derive(serde::Deserialize)]
pub(crate) struct GetTaskResultError<'a> {
    pub error_code: Cow<'a, str>,
}

// Response (success)

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "status", rename_all = "lowercase")]
pub(crate) enum GetTaskResultResponse<'a, T> {
    #[serde(rename_all = "camelCase")]
    Ready(CaptchaSolution<'a, T>),
    Processing,
}
