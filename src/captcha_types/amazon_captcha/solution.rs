use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
pub struct AmazonCaptchaSolution<'a> {
    pub captcha_voucher: Cow<'a, str>,
    pub existing_token: Cow<'a, str>,
}
