use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

/// Represents the data required by the 2captcha API to solve a
/// AmazonCaptcha challenge
///
/// # Example
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     captcha_types::amazon_captcha::AmazonCaptcha
/// };
///
/// # fn main() -> Result<(), captcha_oxide::Error> {
/// let captcha = AmazonCaptcha::builder()
///     .website_url("https://someurl.com/latest")
///     .website_key("AQIDA...wZwdADFLWk7XOA==")
///     .iv("CgAAXFFFFSAAABVk")
///     .context("qoJYgnKsc...aormh/dYYK+Y=")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[proxy_task(with_proxy = "AmazonTask", proxyless = "AmazonTaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::solution::AmazonCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct AmazonCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    /// Value of the `key` parameter you found on the page
    pub(super) website_key: Cow<'a, str>,

    /// Value of the `iv` parameter you found on the page
    pub(super) iv: Cow<'a, str>,

    /// Value of the `context` parameter you found on the page
    pub(super) context: Cow<'a, str>,

    /// The source URL of the `challenge.js` script on the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) challenge_script: Option<Cow<'a, str>>,

    /// The source URL of the `captcha.js` script on the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) captcha_script: Option<Cow<'a, str>>,
}
