use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

/// Represents the data required by the 2captcha API to solve a
/// LeminCaptcha challenge
///
/// # Example
/// ```
/// use captcha_oxide::{CaptchaTask, captcha_types::lemin_captcha::LeminCaptcha};
///
/// # fn main() -> Result<(), captcha_oxide::Error> {
/// let captcha = LeminCaptcha::builder()
///     .website_url("https://2captcha.com/demo/lemin")
///     .captcha_id("CROPPED_3dfdd5c_d1872b526b794d83ba3b365eb15a200b")
///     .div_id("lemin-cropped-captcha")
///     .lemin_api_server_subdomain(Some("api.leminnow.com"))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[proxy_task(with_proxy = "LeminTask", proxyless = "LeminTaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::solution::LeminCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct LeminCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    /// Lemin `captchaId` value. Unique for a website.
    pub(super) captcha_id: Cow<'a, str>,

    /// The `id` of the captcha's parent `div` element
    pub(super) div_id: Cow<'a, str>,

    /// API domain used to load the captcha scripts. Default: `https://api.leminnow.com/`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) lemin_api_server_subdomain: Option<Cow<'a, str>>,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,
}
