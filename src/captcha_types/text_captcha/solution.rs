use std::borrow::Cow;

use captcha_oxide_derive::captcha_solution;

#[captcha_solution]
pub struct TextCaptchaSolution<'a> {
    pub text: Cow<'a, str>,
}
