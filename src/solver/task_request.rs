#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TaskRequest<'a> {
    pub client_key: &'a str,
    pub task_id: u64,
}
