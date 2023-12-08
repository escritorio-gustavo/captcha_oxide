use std::borrow::Cow;
use url::Url;

use captcha_oxide_derive::CaptchaTask;

use crate::proxy::Proxy;

#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::solution::DataDomeCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase", tag = "type", rename = "DataDomeSliderTask")]
pub struct DataDomeCaptcha<'a> {
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) captcha_url: Url,
    pub(super) user_agent: Cow<'a, str>,

    #[serde(flatten)]
    pub(super) proxy: Proxy<'a>,
}
