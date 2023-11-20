use std::borrow::Cow;

use catptcha_oxide_derive::captcha_solution;

#[captcha_solution]
pub struct ReCaptchaSolution<'a> {
    pub g_recaptcha_response: Cow<'a, str>,
    pub token: Cow<'a, str>,
}
