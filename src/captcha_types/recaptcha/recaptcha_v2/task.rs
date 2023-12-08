use captcha_oxide_derive::proxy_task;
use std::borrow::Cow;
use url::Url;

use crate::captcha_types::CaptchaTask;

#[proxy_task(with_proxy = "RecaptchaV2Task", proxyless = "RecaptchaV2TaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::super::solution::ReCaptchaSolution<'a>, crate = crate)]
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
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) recaptcha_data_s_value: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) is_invisible: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    // #[task(builder_type = Option<crate::cookie::Cookies<'a>>, parse_with = { infallible({ path = crate::cookie::Cookies::stringify, parse_ref }) })]
    pub(super) cookies: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) api_domain: Option<Cow<'a, str>>,
}
