use captcha_oxide_derive::{proxy_task, CaptchaTask};
use std::borrow::Cow;
use url::Url;

/// Represents the data required by the 2captcha API to solve a
/// MtCaptcha challenge
///
/// # Example
/// ```
/// use captcha_oxide::{CaptchaTask, captcha_types::mt_captcha::MtCaptcha};
///
/// # fn main() -> Result<(), captcha_oxide::Error> {
/// let captcha = MtCaptcha::builder()
///     .website_url("https://some_url.com")
///     .website_key("SOME_SITE_KEY")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[proxy_task(with_proxy = "MtCaptchaTask", proxyless = "MtCaptchaTaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::solution::MtCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct MtCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    /// The MTCaptcha `sitekey` value found in the page code.
    pub(super) website_key: Cow<'a, str>,
}
