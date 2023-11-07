use std::{borrow::Cow, marker::PhantomData};

use url::Url;

use crate::{
    captcha_types::empty_data::Empty,
    proxy::Proxy,
    type_state::{UrlMissing, WebsitePublicKeyMissing},
    CaptchaTask,
};

use super::{builder::ArkoseLabsCaptchaBuilder, solution::ArkoseLabsCaptchaSolution};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArkoseLabsCaptcha<'a, T = Empty>
where
    T: serde::Serialize,
{
    #[serde(skip)]
    pub(super) _x: PhantomData<T>,

    #[serde(flatten)]
    pub(super) task_type: ArkoseLabsCaptchaTypes<'a>,

    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) website_public_key: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) funcaptcha_api_jssubdomain: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) data: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,
}

#[derive(serde::Serialize)]
#[serde(tag = "type")]
#[catptcha_oxide_derive::from_option]
pub enum ArkoseLabsCaptchaTypes<'a> {
    #[serde(rename = "FunCaptchaTaskProxyless")]
    ProxyLess,

    #[serde(rename = "FunCaptchaTask")]
    WithProxy(Proxy<'a>),
}

impl<'a, T> CaptchaTask for ArkoseLabsCaptcha<'a, T>
where
    T: serde::Serialize,
{
    type Solution = ArkoseLabsCaptchaSolution<'a>;
    type Builder = ArkoseLabsCaptchaBuilder<'a, UrlMissing, WebsitePublicKeyMissing, T>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
