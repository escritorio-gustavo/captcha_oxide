use captcha_oxide_derive::captcha_solution;
use std::borrow::Cow;

#[captcha_solution]
#[serde(rename_all = "camelCase")]
pub struct HCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
    pub resp_key: Cow<'a, str>,
    pub user_agent: Cow<'a, str>,
    pub g_recaptcha_response: Cow<'a, str>,
}
