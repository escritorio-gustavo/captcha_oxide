use catptcha_oxide_derive::proxy_task;
use std::borrow::Cow;
use url::Url;

use crate::{
    captcha_types::{
        recaptcha::{recaptcha_v2::builder::RecaptchaV2Builder, solution::ReCaptchaSolution},
        CaptchaTask,
    },
    type_state::{UrlMissing, WebsiteKeyMissing},
};

#[proxy_task(with_proxy = "RecaptchaV2Task", proxyless = "RecaptchaV2TaskProxyless")]
#[serde(rename_all = "camelCase")]
/// Represents the data required by the 2captcha API to solve a reCaptcha V2
/// challenge
///
/// # Example
/// ```
/// use captcha_oxide::{Error, captcha_types::{CaptchaTask, RecaptchaV2}};
/// use url::Url;
///
/// # fn main() -> Result<(), Error> {
/// let captcha = RecaptchaV2::builder()
///     .website_url(Url::parse("http://someurl.com")?)
///     .website_key("SOME_KEY")
///     .build();
/// # Ok(())
/// # }
/// ```
pub struct RecaptchaV2<'a> {
    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) recaptcha_data_s_value: Option<Cow<'a, str>>,

    pub(super) is_invisible: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) cookies: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) api_domain: Option<Cow<'a, str>>,
}

impl<'a> CaptchaTask for RecaptchaV2<'a> {
    type Solution = ReCaptchaSolution<'a>;
    type Builder = RecaptchaV2Builder<'a, UrlMissing, WebsiteKeyMissing>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
