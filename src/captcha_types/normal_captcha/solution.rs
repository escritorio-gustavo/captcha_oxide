use std::borrow::Cow;

use catptcha_oxide_derive::captcha_solution;

#[captcha_solution]
pub struct NormalCaptchaSolution<'a> {
    pub text: Cow<'a, str>,
}
