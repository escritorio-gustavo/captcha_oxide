use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

#[proxy_task(with_proxy = "GeeTestTask", proxyless = "GeeTestTaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::solution::GeeTestV3Solution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct GeeTestV3<'a> {
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,
    pub(super) gt: Cow<'a, str>,
    pub(super) challenge: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) geetest_api_server_subdomain: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,
}
