use captcha_oxide_derive::{proxy_task, CaptchaTask};
use std::borrow::Cow;
use url::Url;

#[proxy_task(with_proxy = "MtCaptchaTask", proxyless = "MtCaptchaTaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::solution::MtCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct MtCaptcha<'a> {
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,
}
