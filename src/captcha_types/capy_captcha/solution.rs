use std::borrow::Cow;

use catptcha_oxide_derive::captcha_solution;

#[captcha_solution]
pub struct CapyCaptchaSolution<'a> {
    #[serde(rename = "captchakey")]
    pub captcha_key: Cow<'a, str>,

    #[serde(rename = "challengekey")]
    pub challenge_key: Cow<'a, str>,
    pub answer: Cow<'a, str>,

    #[serde(rename = "respKey")]
    pub resp_key: Cow<'a, str>,
}
