use std::borrow::Cow;

use captcha_oxide_derive::captcha_solution;

#[captcha_solution]
pub struct AudioCaptchaSolution<'a> {
    pub text: Cow<'a, str>,
}
