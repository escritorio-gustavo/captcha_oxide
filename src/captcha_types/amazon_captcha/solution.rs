use std::borrow::Cow;

use captcha_oxide_derive::captcha_solution;

#[captcha_solution]
pub struct AmazonCaptchaSolution<'a> {
    pub captcha_voucher: Cow<'a, str>,
    pub existing_token: Cow<'a, str>,
}
