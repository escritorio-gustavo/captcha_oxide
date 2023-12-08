use std::borrow::Cow;
use url::Url;

use crate::CaptchaTask;

#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::super::solution::ReCaptchaSolution<'a>, crate = crate)]
#[serde(
    rename_all = "camelCase",
    tag = "type",
    rename = "RecaptchaV3TaskProxyless"
)]
/// Represents the data required by the 2captcha API to solve a reCaptcha V3
/// challenge
///
/// # Example
/// ```
/// use captcha_oxide::{Error, captcha_types::{CaptchaTask, RecaptchaV3}};
/// use url::Url;
///
/// # fn main() -> Result<(), Error> {
/// let captcha = RecaptchaV3::builder()
///     .website_url(Url::parse("http://someurl.com")?)
///     .website_key("SOME_KEY")
///     .min_score(0.3)
///     .build();
/// # Ok(())
/// # }
/// ```
pub struct RecaptchaV3<'a> {
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,
    pub(super) min_score: f32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) page_action: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) is_enterprise: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) api_domain: Option<Cow<'a, str>>,
}
