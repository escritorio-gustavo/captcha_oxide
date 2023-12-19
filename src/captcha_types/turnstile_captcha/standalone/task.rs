use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

/// Represents the data required by the 2captcha API to solve a Turnstile
/// standalone challenge
///
/// # Example
/// ```
/// use captcha_oxide::{
///     Error,
///     CaptchaTask,
///     captcha_types::turnstile_captcha::TurnstileStandaloneCaptcha,
/// };
///
/// # fn main() -> Result<(), Error> {
/// let captcha = TurnstileStandaloneCaptcha::builder()
///     .website_url("http://someurl.com")
///     .website_key("SOME_KEY")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[proxy_task(with_proxy = "TurnstileTask", proxyless = "TurnstileTaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::super::solution::TurnstileCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct TurnstileStandaloneCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,

    /// Turnstile sitekey. Can be found inside the `data-sitekey` property of
    /// the Turnstile `div` element
    pub(super) website_key: Cow<'a, str>,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,
}
