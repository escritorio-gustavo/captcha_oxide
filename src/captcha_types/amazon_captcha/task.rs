use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

#[proxy_task(with_proxy = "AmazonTask", proxyless = "AmazonTaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::solution::AmazonCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct AmazonCaptcha<'a> {
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,
    pub(super) iv: Cow<'a, str>,
    pub(super) context: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) challenge_script: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) captcha_script: Option<Cow<'a, str>>,
}
