use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::{
    captcha_types::{empty_data::Empty, geetest::geetest_v4::type_state::*},
    CaptchaTask,
};

use super::{builder::GeeTestV4Builder, solution::GeeTestV4Solution};

#[proxy_task(with_proxy = "GeeTestTask", proxyless = "GeeTestTaskProxyless")]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeeTestV4<'a, T = Empty>
where
    T: serde::Serialize,
{
    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) gt: Cow<'a, str>,
    pub(super) challenge: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) geetest_api_server_subdomain: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,
    pub(super) init_parameters: InitParameters<'a, T>,

    pub(super) version: u8,
}

#[derive(Debug, serde::Serialize)]
pub struct InitParameters<'a, T> {
    pub(super) captcha_id: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub(super) data: Option<T>,
}

impl<'a, T> CaptchaTask for GeeTestV4<'a, T>
where
    T: serde::Serialize,
{
    type Solution = GeeTestV4Solution<'a>;
    type Builder =
        GeeTestV4Builder<'a, UrlMissing, GtMissing, ChallengeMissing, CaptchaIdMissing, T>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
