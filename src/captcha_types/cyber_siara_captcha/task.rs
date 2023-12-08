use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

#[proxy_task(
    with_proxy = "AntiCyberSiAraTask",
    proxyless = "AntiCyberSiAraTaskProxyless",
    crate = crate
)]
#[derive(CaptchaTask, serde::Serialize)]
#[task(timeout = 20, solution = super::solution::CyberSiARACaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct CyberSiARACaptcha<'a> {
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    pub(super) slide_master_url_id: Cow<'a, str>,
    pub(super) user_agent: Cow<'a, str>,
}
