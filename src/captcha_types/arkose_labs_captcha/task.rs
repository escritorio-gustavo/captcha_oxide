use std::{borrow::Cow, marker::PhantomData};

use catptcha_oxide_derive::proxy_task;
use url::Url;

use crate::{
    captcha_types::empty_data::Empty,
    type_state::{UrlMissing, WebsitePublicKeyMissing},
    CaptchaTask,
};

use super::{builder::ArkoseLabsCaptchaBuilder, solution::ArkoseLabsCaptchaSolution};

#[proxy_task(with_proxy = "FunCaptchaTask", proxyless = "FunCaptchaTaskProxyless")]
#[serde(rename_all = "camelCase")]
pub struct ArkoseLabsCaptcha<'a, T = Empty>
where
    T: serde::Serialize,
{
    #[serde(skip)]
    pub(super) _x: PhantomData<T>,

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
