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

use super::RecaptchaV2EnterpriseBuilder;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaV2Enterprise<'a, T>
where
    T: serde::Serialize,
{
    #[serde(flatten)]
    pub(super) task_type: RecaptchaV2EnterpriseTypes<'a>,

    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) enterprise_payload: Option<T>,

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
pub enum RecaptchaV2EnterpriseTypes<'a> {
    #[serde(rename = "RecaptchaV2EnterpriseTaskProxyless")]
    ProxyLess,

    #[serde(rename = "RecaptchaV2EnterpriseTask")]
    WithProxy(Proxy<'a>),
}

impl<'a, T> CaptchaTask for RecaptchaV2Enterprise<'a, T>
where
    T: serde::Serialize,
{
    type Solution = ReCaptchaSolution<'a>;
    type Builder = RecaptchaV2EnterpriseBuilder<'a, NoUrlProvided, NoWebsiteKeyProvided, T>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
