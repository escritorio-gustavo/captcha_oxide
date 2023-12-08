use std::{borrow::Cow, marker::PhantomData};

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::{captcha_types::empty_data::Empty, CaptchaTask};

#[proxy_task(with_proxy = "FunCaptchaTask", proxyless = "FunCaptchaTaskProxyless", crate = crate)]
#[derive(CaptchaTask, serde::Serialize)]
#[task(timeout = 20, crate = crate, solution = super::solution::ArkoseLabsCaptchaSolution<'a>)]
#[serde(rename_all = "camelCase")]
pub struct ArkoseLabsCaptcha<'a, T = Empty>
where
    T: serde::Serialize,
{
    #[serde(skip)]
    #[task(default = Default::default)]
    pub(super) _x: PhantomData<T>,

    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse })})]
    pub(super) website_url: Url,
    pub(super) website_public_key: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) funcaptcha_api_jssubdomain: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[task(builder_type = Option<T>, parse_with = { fallible({ path = serde_json::to_string, parse_ref }) })]
    pub(super) data: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,
}
