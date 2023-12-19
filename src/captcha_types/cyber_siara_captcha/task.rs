use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

/// Represents the data required by the 2captcha API to solve a
/// CyberSiARACaptcha challenge
///
/// # Example
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     captcha_types::cyber_siara_captcha::CyberSiARACaptcha,
/// };
///
/// # fn main() -> Result<(), captcha_oxide::Error> {
/// let captcha = CyberSiARACaptcha::builder()
///     .website_url("http://some_url.com")
///     .slide_master_url_id("OXR2LVNvCuXykkZbB8KZIfh162sNT8S2")
///     .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[proxy_task(
    with_proxy = "AntiCyberSiAraTask",
    proxyless = "AntiCyberSiAraTaskProxyless",
    crate = crate
)]
#[derive(CaptchaTask, serde::Serialize)]
#[task(timeout = 20, solution = super::solution::CyberSiARACaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct CyberSiARACaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    /// The value of the `MasterUrlId` parameter obtained from the request
    /// to the endpoint `API/CyberSiara/GetCyberSiara`.
    #[serde(rename = "SlideMasterUrlId")]
    pub(super) slide_master_url_id: Cow<'a, str>,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    pub(super) user_agent: Cow<'a, str>,
}
