use std::borrow::Cow;

use catptcha_oxide_derive::proxy_task;
use url::Url;

use crate::{
    type_state::{UrlMissing, WebsiteKeyMissing},
    CaptchaTask,
};

use super::{builder::CapyCaptchaBuilder, solution::CapyCaptchaSolution};

#[proxy_task(with_proxy = "CapyTask", proxyless = "CapyTaskProxyless")]
pub struct CapyCaptcha<'a> {
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,
}

impl<'a> CaptchaTask for CapyCaptcha<'a> {
    type Solution = CapyCaptchaSolution<'a>;
    type Builder = CapyCaptchaBuilder<'a, UrlMissing, WebsiteKeyMissing>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
