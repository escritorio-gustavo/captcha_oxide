use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

/// Represents the data required by the 2captcha API to solve a
/// GeeTestV3 challenge
///
/// # Example
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     captcha_types::geetest::GeeTestV3
/// };
///
/// # fn main() -> Result<(), captcha_oxide::Error> {
/// let captcha = GeeTestV3::builder()
///     .website_url("https://someurl.com/latest")
///     .gt("81388ea1fc187e0c335c0a8907ff2625")
///     .challenge("2e2f0f65240058b683cb6ea21c303eea6n")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[proxy_task(with_proxy = "GeeTestTask", proxyless = "GeeTestTaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::solution::GeeTestV3Solution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct GeeTestV3<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    /// GeeTest `gt` value.
    pub(super) gt: Cow<'a, str>,

    /// GeeTest `challenge` value.
    pub(super) challenge: Cow<'a, str>,

    /// Custom GeeTest API domain, for example: `api-na.geetest.com`.
    /// Can be defined inside `initGeetest` call. Also you can check
    /// the domain used to load the scripts, the default domain is
    /// `api.geetest.com`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) geetest_api_server_subdomain: Option<Cow<'a, str>>,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,
}
