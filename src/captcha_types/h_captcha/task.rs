use std::borrow::Cow;
use url::Url;

use crate::{
    captcha_types::{
        type_state::{NoUrlProvided, NoWebsiteKeyProvided},
        CaptchaTask,
    },
    proxy::Proxy,
};

use super::{solution::HCaptchaSolution, HCaptchaBuilder};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HCaptcha<'a, T>
where
    T: serde::Serialize,
{
    #[serde(flatten)]
    pub(super) task_type: HCaptchaTypes<'a>,

    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,
    pub(super) is_invisible: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) enterprise_payload: Option<T>,
}

#[derive(serde::Serialize)]
#[serde(tag = "type")]
pub enum HCaptchaTypes<'a> {
    #[serde(rename = "HCaptchaTaskProxyless")]
    ProxyLess,

    #[serde(rename = "HCaptchaTask")]
    WithProxy(Proxy<'a>),
}

impl<'a, T> CaptchaTask for HCaptcha<'a, T>
where
    T: serde::Serialize,
{
    type Solution = HCaptchaSolution<'a>;
    type Builder = HCaptchaBuilder<'a, NoUrlProvided, NoWebsiteKeyProvided, T>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
