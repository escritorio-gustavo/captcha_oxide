use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::{
    type_state::{UrlMissing, WebsiteKeyMissing},
    CaptchaTask,
};

use super::{
    builder::AmazonCaptchaBuilder,
    solution::AmazonCaptchaSolution,
    type_state::{ContextMissing, IvMissing},
};

#[proxy_task(with_proxy = "AmazonTask", proxyless = "AmazonTaskProxyless")]
#[serde(rename_all = "camelCase")]
pub struct AmazonCaptcha<'a> {
    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,
    pub(super) iv: Cow<'a, str>,
    pub(super) context: Cow<'a, str>,
    pub(super) challenge_script: Option<Cow<'a, str>>,
    pub(super) captcha_script: Option<Cow<'a, str>>,
}

impl<'a> CaptchaTask for AmazonCaptcha<'a> {
    type Solution = AmazonCaptchaSolution<'a>;
    type Builder =
        AmazonCaptchaBuilder<'a, UrlMissing, WebsiteKeyMissing, IvMissing, ContextMissing>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
