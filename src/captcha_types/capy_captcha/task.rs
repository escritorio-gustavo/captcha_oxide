use std::borrow::Cow;

use catptcha_oxide_derive::from_option;
use url::Url;

use crate::{
    proxy::Proxy,
    type_state::{UrlMissing, WebsiteKeyMissing},
    CaptchaTask,
};

use super::{builder::CapyCaptchaBuilder, solution::CapyCaptchaSolution};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CapyCaptcha<'a> {
    #[serde(flatten)]
    pub(super) task_type: CapyCaptchaTypes<'a>,

    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,
}

#[derive(serde::Serialize)]
#[serde(tag = "type")]
#[from_option]
pub enum CapyCaptchaTypes<'a> {
    #[serde(rename = "CapyTaskProxyless")]
    ProxyLess,

    #[serde(rename = "CapyTask")]
    WithProxy(Proxy<'a>),
}

impl<'a> CaptchaTask for CapyCaptcha<'a> {
    type Solution = CapyCaptchaSolution<'a>;
    type Builder = CapyCaptchaBuilder<'a, UrlMissing, WebsiteKeyMissing>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
