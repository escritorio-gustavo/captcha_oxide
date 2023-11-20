use std::borrow::Cow;

use catptcha_oxide_derive::proxy_task;
use url::Url;

use crate::{type_state::UrlMissing, CaptchaTask};

use super::{
    builder::KeyCaptchaBuilder,
    solution::KeyCaptchaSolution,
    type_state::{SessionIdMissing, UserIdMissing, WebServerSign2Missing, WebServerSignMissing},
};

#[proxy_task(with_proxy = "KeyCaptchaTask", proxyless = "KeyCaptchaTaskProxyless")]
pub struct KeyCaptcha<'a> {
    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,

    #[serde(rename = "s_s_c_user_id")]
    pub(super) user_id: u32,

    #[serde(rename = "s_s_c_session_id")]
    pub(super) session_id: Cow<'a, str>,

    #[serde(rename = "s_s_c_web_server_sign")]
    pub(super) web_server_sign: Cow<'a, str>,

    #[serde(rename = "s_s_c_web_server_sign2")]
    pub(super) web_server_sign2: Cow<'a, str>,
}

impl<'a> CaptchaTask for KeyCaptcha<'a> {
    type Solution = KeyCaptchaSolution<'a>;
    type Builder = KeyCaptchaBuilder<
        'a,
        UrlMissing,
        UserIdMissing,
        SessionIdMissing,
        WebServerSignMissing,
        WebServerSign2Missing,
    >;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
