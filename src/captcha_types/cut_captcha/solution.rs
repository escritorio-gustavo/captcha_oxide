use std::borrow::Cow;

use captcha_oxide_derive::captcha_solution;

#[captcha_solution]
pub struct CutCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}
