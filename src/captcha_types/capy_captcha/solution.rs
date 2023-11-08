use std::borrow::Cow;

use crate::CaptchaSolution;

#[derive(serde::Deserialize, CaptchaSolution)]

pub struct CapyCaptchaSolution<'a> {
    #[serde(default = "Default::default")]
    task_id: u64,

    #[serde(rename = "captchakey")]
    pub captcha_key: Cow<'a, str>,

    #[serde(rename = "challengekey")]
    pub challenge_key: Cow<'a, str>,
    pub answer: Cow<'a, str>,

    #[serde(rename = "respKey")]
    pub resp_key: Cow<'a, str>,
}
