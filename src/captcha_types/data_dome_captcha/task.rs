use std::borrow::Cow;
use url::Url;

use captcha_oxide_derive::CaptchaTask;

use crate::proxy::Proxy;

#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20)]
#[serde(rename_all = "camelCase", tag = "type", rename = "DataDomeSliderTask")]
pub struct DataDomeCaptcha<'a> {
    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) captcha_url: Url,
    pub(super) user_agent: Cow<'a, str>,

    #[serde(flatten)]
    pub(super) proxy: Proxy<'a>,
}
