use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

#[proxy_task(with_proxy = "CapyTask", proxyless = "CapyTaskProxyless", crate = crate)]
#[derive(CaptchaTask, serde::Serialize)]
#[task(timeout = 20, solution = super::solution::CapyCaptchaSolution::<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct CapyCaptcha<'a> {
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,
}
