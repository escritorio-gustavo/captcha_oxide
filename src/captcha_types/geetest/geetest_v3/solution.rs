use std::borrow::Cow;

use crate::captcha_types::CaptchaSolution;

#[derive(serde::Deserialize, CaptchaSolution)]
#[serde(rename_all = "camelCase")]
pub struct GeeTestV3Solution<'a> {
    #[serde(default = "Default::default")]
    task_id: u64,

    pub challenge: Cow<'a, str>,
    pub validate: Cow<'a, str>,
    pub seccode: Cow<'a, str>,
}
