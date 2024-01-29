use captcha_oxide_derive::proxy_task;
use std::borrow::Cow;
use url::Url;

use crate::CaptchaTask;

/// Represents the data required by the 2captcha API to solve a
/// FriendlyCaptcha challenge
///
/// # Example
/// ```
/// use captcha_oxide::{CaptchaTask, captcha_types::friendly_captcha::FriendlyCaptcha};
///
/// # fn main() -> Result<(), captcha_oxide::Error> {
/// let captcha = FriendlyCaptcha::builder()
///     .website_url("http://someurl.com")
///     .website_key("SOME_KEY")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[proxy_task(with_proxy = "FriendlyCaptchaTask", proxyless = "FriendlyCaptchaTaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::solution::FriendlyCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct FriendlyCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    pub(super) website_key: Cow<'a, str>,
}
