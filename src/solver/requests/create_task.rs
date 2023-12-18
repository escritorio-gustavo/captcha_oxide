use url::Url;

use crate::{solver::language_pool::LanguagePool, CaptchaTask};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateTaskRequest<'a, T>
where
    T: CaptchaTask,
{
    pub client_key: &'a str,
    pub task: &'a T,
    pub soft_id: u16,
    pub language_pool: LanguagePool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<&'a Url>,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum CreateTaskResponse {
    #[serde(rename_all = "camelCase")]
    TaskCreated { task_id: u64 },

    #[serde(rename_all = "camelCase")]
    Error { error_code: Box<str> },
}
