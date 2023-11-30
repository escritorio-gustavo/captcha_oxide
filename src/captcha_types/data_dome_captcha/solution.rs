use std::borrow::Cow;

use captcha_oxide_derive::captcha_solution;

#[captcha_solution]
pub struct DataDomeCaptchaSolution<'a> {
    pub cookie: Cow<'a, str>,
}
