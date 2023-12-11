use std::borrow::Cow;

use captcha_oxide_derive::captcha_solution;

#[captcha_solution]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaSolution<'a> {
    pub g_recaptcha_response: Cow<'a, str>,
    pub token: Cow<'a, str>,
}
