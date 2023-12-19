use std::borrow::Cow;

use captcha_oxide_derive::{proxy_task, CaptchaTask};
use url::Url;

/// Token-based method to bypass Cutcaptcha.
/// The token received must be set as the value attribute of
/// the `input#cap_token` element and/or passed to the callback function.
///
/// # Example
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     captcha_types::cut_captcha::CutCaptcha,
/// };
///
/// # fn main() -> Result<(), captcha_oxide::Error> {
/// let captcha = CutCaptcha::builder()
///     .website_url("http://some_url.com")
///     .misery_key("a1488b66da00bf332a1488993a5443c79047e752")
///     .api_key("SAb83IIB")
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
#[task(timeout = 20, solution = super::solution::CutCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct CutCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    /// The value of the `CUTCAPTCHA_MISERY_KEY` variable defined on the page.
    pub(super) misery_key: Cow<'a, str>,

    /// The value of the `data-apikey` attribute in the `iframe`'s body.
    /// Also the name of the javascript file included on the page
    pub(super) api_key: Cow<'a, str>,
}
