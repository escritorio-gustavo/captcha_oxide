use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

#[proxy_task(with_proxy = "KeyCaptchaTask", proxyless = "KeyCaptchaTaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::solution::KeyCaptchaSolution<'a>, crate = crate)]
pub struct KeyCaptcha<'a> {
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    #[serde(rename = "s_s_c_user_id")]
    pub(super) user_id: u32,

    #[serde(rename = "s_s_c_session_id")]
    pub(super) session_id: Cow<'a, str>,

    #[serde(rename = "s_s_c_web_server_sign")]
    pub(super) web_server_sign: Cow<'a, str>,

    #[serde(rename = "s_s_c_web_server_sign2")]
    pub(super) web_server_sign2: Cow<'a, str>,
}
