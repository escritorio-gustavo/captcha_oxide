use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

#[proxy_task(with_proxy = "TurnstileTask", proxyless = "TurnstileTaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::super::solution::TurnstileCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct TurnstileStandaloneCaptcha<'a> {
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,
}
