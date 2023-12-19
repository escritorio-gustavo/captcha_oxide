use std::borrow::Cow;
use url::Url;

use captcha_oxide_derive::CaptchaTask;

use crate::proxy::Proxy;

/// Token-based method for automated solving of DataDome. \
/// To solve the DataDome captcha, you **must** use a proxy. \
/// \
/// ## Attention
/// You need to check the value of the parameter `t` in `captcha_url`,
/// the value of `t` must be equal to `fe`. \
/// If `t=bv`, it means that your ip is banned by the captcha and you
/// need to change the ip address. \
/// \
/// ## Attention
/// You need to monitor the quality of the proxy used. If your proxy
/// is blocked by DataDome you will receive the following solving errors:
/// * [crate::Error::TwoCaptchaError(crate::solver::error::SolveError::ProxyConnectionFailed)]
/// * [crate::Error::TwoCaptchaError(crate::solver::error::SolveError::UnsolvableCaptcha)] \
/// In which case you need to change the proxy server used.
///
/// # Example
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     captcha_types::data_dome_captcha::DataDomeCaptcha,
///     proxy::{Proxy, Address, ProxyType}
/// };
///
/// # fn main() -> Result<(), captcha_oxide::Error> {
/// let captcha = DataDomeCaptcha::builder()
///     .website_url("https://some_url.com/")
///     .captcha_url("https://other_url.com/")
///     .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36")
///     .proxy(Proxy {
///         proxy_type: ProxyType::Http,
///         proxy_address: Address::HostName("some.proxy.com".into()),
///         proxy_port: "1234".into(),
///         proxy_login: None,
///         proxy_password: None,
///     })
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::solution::DataDomeCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase", tag = "type", rename = "DataDomeSliderTask")]
pub struct DataDomeCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    /// The value of the `src` parameter for the `iframe` element containing
    /// the captcha on the page.
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) captcha_url: Url,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    pub(super) user_agent: Cow<'a, str>,

    /// Proxy connection data
    #[serde(flatten)]
    pub(super) proxy: Proxy<'a>,
}
