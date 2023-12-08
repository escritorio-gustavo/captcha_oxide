use std::borrow::Cow;

use captcha_oxide_derive::{proxy_task, CaptchaTask};
use url::Url;

#[proxy_task(
    with_proxy = "AntiCyberSiAraTask",
    proxyless = "AntiCyberSiAraTaskProxyless",
    crate = crate
)]
#[derive(CaptchaTask, serde::Serialize)]
#[task(timeout = 20, solution = super::solution::CutCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct CutCaptcha<'a> {
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,
    pub(super) misery_key: Cow<'a, str>,
    pub(super) api_key: Cow<'a, str>,
}
