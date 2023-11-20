use std::borrow::Cow;

use catptcha_oxide_derive::proxy_task;
use url::Url;

use crate::{
    captcha_types::turnstile_captcha::solution::TurnstileCaptchaSolution,
    type_state::{UrlMissing, WebsiteKeyMissing},
    CaptchaTask,
};

use super::{
    builder::TurnstileChallengePageCaptchaBuilder,
    type_state::{ActionMissing, DataMissing, PageDataMissing, UserAgentMissing},
};

#[proxy_task(with_proxy = "TurnstileTask", proxyless = "TurnstileTaskProxyless")]
pub struct TurnstileChallengePageCaptcha<'a> {
    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,
    pub(super) user_agent: Cow<'a, str>,
    pub(super) action: Cow<'a, str>,
    pub(super) data: Cow<'a, str>,
    pub(super) page_data: Cow<'a, str>,
}

impl<'a> CaptchaTask for TurnstileChallengePageCaptcha<'a> {
    type Solution = TurnstileCaptchaSolution<'a>;
    type Builder = TurnstileChallengePageCaptchaBuilder<
        'a,
        UrlMissing,
        WebsiteKeyMissing,
        UserAgentMissing,
        ActionMissing,
        DataMissing,
        PageDataMissing,
    >;

    fn get_timeout(&self) -> std::time::Duration {
        todo!()
    }
}
