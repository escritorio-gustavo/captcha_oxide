use std::borrow::Cow;

use captcha_oxide_derive::captcha_solution;

#[captcha_solution]
pub struct TurnstileCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
    pub user_agent: Cow<'a, str>,
}
