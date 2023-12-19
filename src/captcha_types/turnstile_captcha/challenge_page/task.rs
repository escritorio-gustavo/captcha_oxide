use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

/// Represents the data required by the 2captcha API to solve a reCaptcha V2
/// challenge
///
/// # Example
/// ```
/// use captcha_oxide::{
///     Error,
///     CaptchaTask,
///     captcha_types::turnstile_captcha::TurnstileChallengePageCaptcha,
/// };
///
/// # fn main() -> Result<(), Error> {
/// let captcha = TurnstileChallengePageCaptcha::builder()
///     .website_url("http://someurl.com")
///     .website_key("SOME_KEY")
///     .action("managed")
///     .data("80001aa1affffc21")
///     .page_data("3gAFo2l...55NDFPRFE9")
///     .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[proxy_task(with_proxy = "TurnstileTask", proxyless = "TurnstileTaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::super::solution::TurnstileCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct TurnstileChallengePageCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    /// Turnstile sitekey. Can be found inside the `data-sitekey` property of
    /// the Turnstile `div` element
    pub(super) website_key: Cow<'a, str>,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    pub(super) user_agent: Cow<'a, str>,

    /// The value of `action` parameter of the `turnstile.render` call
    pub(super) action: Cow<'a, str>,

    /// The value of `cData` parameter of the `turnstile.render` call
    pub(super) data: Cow<'a, str>,

    /// The value of `chlPageData` parameter of the `turnstile.render` call
    pub(super) page_data: Cow<'a, str>,
}
