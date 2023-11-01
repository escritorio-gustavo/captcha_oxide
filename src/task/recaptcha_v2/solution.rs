use catptcha_oxide_derive::Solution;
use std::borrow::Cow;

use crate::task::Solution;

#[derive(serde::Deserialize, Debug, Solution)]
#[serde(rename_all = "camelCase")]
pub struct ReCaptchaV2Solution<'a> {
    #[serde(default = "Default::default")]
    pub(crate) task_id: u64,
    pub g_recaptcha_response: Cow<'a, str>,
    pub token: Cow<'a, str>,
}
