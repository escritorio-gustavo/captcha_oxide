use std::borrow::Cow;

use catptcha_oxide_derive::proxy_task;
use url::Url;

use crate::{type_state::UrlMissing, CaptchaTask};

use super::{
    builder::LeminCaptchaBuilder,
    solution::LeminCaptchaSolution,
    type_state::{CaptchaIdMissing, DivIdMissing},
};

#[proxy_task(with_proxy = "LeminTask", proxyless = "LeminTaskProxyless")]
#[serde(rename_all = "camelCase")]
pub struct LeminCaptcha<'a> {
    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,

    pub(super) captcha_id: Cow<'a, str>,
    pub(super) div_id: Cow<'a, str>,
    pub(super) lemin_api_server_subdomain: Option<Cow<'a, str>>,
    pub(super) user_agent: Option<Cow<'a, str>>,
}

impl<'a> CaptchaTask for LeminCaptcha<'a> {
    type Solution = LeminCaptchaSolution<'a>;
    type Builder = LeminCaptchaBuilder<'a, UrlMissing, CaptchaIdMissing, DivIdMissing>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
