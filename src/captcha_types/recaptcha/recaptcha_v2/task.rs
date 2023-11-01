use std::borrow::Cow;
use url::Url;

use crate::{
    captcha_types::{
        recaptcha::{
            solution::ReCaptchaSolution,
            type_state::{NoUrlProvided, NoWebsiteKeyProvided},
        },
        CaptchaTask,
    },
    proxy::Proxy,
};

use super::RecaptchaV2Builder;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaV2<'a> {
    #[serde(flatten)]
    pub(super) task_type: RecaptchaV2Types<'a>,

    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) recaptcha_data_s_value: Option<Cow<'a, str>>,

    pub(super) is_invisible: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) cookies: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) api_domain: Option<Cow<'a, str>>,
}

#[derive(serde::Serialize)]
#[serde(tag = "type")]
pub enum RecaptchaV2Types<'a> {
    #[serde(rename = "RecaptchaV2TaskProxyless")]
    ProxyLess,

    #[serde(rename = "RecaptchaV2Task")]
    WithProxy(Proxy<'a>),
}

impl<'a> CaptchaTask for RecaptchaV2<'a> {
    type Solution = ReCaptchaSolution<'a>;
    type Builder = RecaptchaV2Builder<'a, NoUrlProvided, NoWebsiteKeyProvided>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
