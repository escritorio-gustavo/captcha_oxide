use std::borrow::Cow;

use catptcha_oxide_derive::proxy_task;
use url::Url;

use crate::{
    captcha_types::turnstile_captcha::solution::TurnstileCaptchaSolution,
    type_state::{UrlMissing, WebsiteKeyMissing},
    CaptchaTask,
};

use super::builder::TurnstileStandaloneCaptchaBuilder;

#[proxy_task(with_proxy = "TurnstileTask", proxyless = "TurnstileTaskProxyless")]
pub struct TurnstileStandaloneCaptcha<'a> {
    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,
    pub(super) user_agent: Option<Cow<'a, str>>,
}

impl<'a> CaptchaTask for TurnstileStandaloneCaptcha<'a> {
    type Solution = TurnstileCaptchaSolution<'a>;
    type Builder = TurnstileStandaloneCaptchaBuilder<'a, UrlMissing, WebsiteKeyMissing>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
