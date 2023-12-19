use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

/// Represents the data required by the 2captcha API to solve a
/// KeyCaptcha challenge
///
/// # Example
/// ```
/// use captcha_oxide::{CaptchaTask, captcha_types::key_captcha::KeyCaptcha};
///
/// # fn main() -> Result<(), captcha_oxide::Error> {
/// let captcha = KeyCaptcha::builder()
///     .website_url("https://2captcha.com/demo/keycaptcha")
///     .user_id(184015_u32)
///     .session_id("8510374722aa3f99a7199d306865afb2")
///     .web_server_sign("bed1536559a1cab72ecd0e28e89c431c")
///     .web_server_sign2("104ac902450db8362ce5fc11e841ee47")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[proxy_task(with_proxy = "KeyCaptchaTask", proxyless = "KeyCaptchaTaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::solution::KeyCaptchaSolution<'a>, crate = crate)]
pub struct KeyCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    /// The value of the `s_s_c_user_id` parameter found on page
    #[serde(rename = "s_s_c_user_id")]
    pub(super) user_id: u32,

    /// The value of the `s_s_c_session_id` parameter found on page
    #[serde(rename = "s_s_c_session_id")]
    pub(super) session_id: Cow<'a, str>,

    /// The value of the `s_s_c_web_server_sign` parameter found on page
    #[serde(rename = "s_s_c_web_server_sign")]
    pub(super) web_server_sign: Cow<'a, str>,

    /// The value of the `s_s_c_web_server_sign2` parameter found on page
    #[serde(rename = "s_s_c_web_server_sign2")]
    pub(super) web_server_sign2: Cow<'a, str>,
}
