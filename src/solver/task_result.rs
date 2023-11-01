#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(super) enum TaskResultResponse<T> {
    Success(TaskResult<T>),

    #[serde(rename_all = "camelCase")]
    Error {
        error_code: Box<str>,
    },
}

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "status", rename_all = "lowercase")]
pub(super) enum TaskResult<T> {
    Ready { solution: T },
    Processing,
}
