use crate::captcha_types::Solution;
use std::borrow::Cow;

#[derive(serde::Deserialize, Solution)]
#[serde(rename_all = "camelCase")]
pub struct HCaptchaSolution<'a> {
    pub(crate) task_id: u64,
    pub token: Cow<'a, str>,
    pub resp_key: Cow<'a, str>,
    pub user_agent: Cow<'a, str>,
    pub g_recaptcha_response: Cow<'a, str>,
}
