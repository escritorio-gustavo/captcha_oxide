use std::borrow::Cow;

use crate::captcha_types::CaptchaSolution;

#[derive(serde::Deserialize, CaptchaSolution)]
#[serde(rename_all = "camelCase")]
pub struct TurnstileCaptchaSolution<'a> {
    #[serde(default = "Default::default")]
    task_id: u64,

    pub token: Cow<'a, str>,
    pub user_agent: Cow<'a, str>,
}
