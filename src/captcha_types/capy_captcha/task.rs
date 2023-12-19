use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

/// Represents the data required by the 2captcha API to solve a
/// CapyCaptcha puzzle
///
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     captcha_types::capy_captcha::CapyCaptcha
/// };
///
/// # fn main() -> Result<(), captcha_oxide::Error> {
/// let captcha = CapyCaptcha::builder()
///     .website_url("http://some_url.com")
///     .website_key("PUZZLE_Abc1dEFghIJKLM2no34P56q7rStu8v")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[proxy_task(with_proxy = "CapyTask", proxyless = "CapyTaskProxyless", crate = crate)]
#[derive(CaptchaTask, serde::Serialize)]
#[task(timeout = 20, solution = super::solution::CapyCaptchaSolution::<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct CapyCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    /// Capy Puzzle Captcha `captchakey`.
    pub(super) website_key: Cow<'a, str>,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,
}
