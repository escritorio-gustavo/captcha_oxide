use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

#[proxy_task(with_proxy = "TurnstileTask", proxyless = "TurnstileTaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::super::solution::TurnstileCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct TurnstileChallengePageCaptcha<'a> {
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,
    pub(super) user_agent: Cow<'a, str>,
    pub(super) action: Cow<'a, str>,
    pub(super) data: Cow<'a, str>,
    pub(super) page_data: Cow<'a, str>,
}
