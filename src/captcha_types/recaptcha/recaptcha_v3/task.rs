use std::borrow::Cow;
use url::Url;

use crate::CaptchaTask;

/// Represents the data required by the 2captcha API to solve a reCaptcha V3
/// challenge
///
/// # Example
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     Error,
///     captcha_types::recaptcha::RecaptchaV3,
/// };
///
/// # fn main() -> Result<(), Error> {
/// let captcha = RecaptchaV3::builder()
///     .website_url("http://someurl.com")
///     .website_key("SOME_KEY")
///     .min_score(0.3)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::super::solution::RecaptchaSolution<'a>, crate = crate)]
#[serde(
    rename_all = "camelCase",
    tag = "type",
    rename = "RecaptchaV3TaskProxyless"
)]
pub struct RecaptchaV3<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    /// reCAPTCHA sitekey. Can be found inside `data-sitekey` property of the reCAPTCHA
    /// `div` element or inside the `k` parameter of the requests to the reCAPTHCHA API.
    /// You can also use [this script](https://gist.github.com/2captcha/2ee70fa1130e756e1693a5d4be4d8c70) to find the value
    pub(super) website_key: Cow<'a, str>,

    /// Required score value. Recommended values are `0.3`, `0.7` and `0.9`
    pub(super) min_score: f32,

    /// Action parameter value. The value is set by website owner inside
    /// the `data-action` property of the reCAPTCHA `div` element or passed
    /// inside the options object of the `execute` method call,
    /// like `grecaptcha.execute('websiteKey', { action: 'myAction' })`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) page_action: Option<Cow<'a, str>>,

    /// Indicates the usage of the Enterprise version of reCAPTCHA.
    /// You can identify it by the `enterprise.js` script being used instead
    /// of `api.js` or by the `grecaptcha.enterprise.execute` call being used
    /// instead of `grecaptcha.execute`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) is_enterprise: Option<bool>,

    /// Domain used to load the captcha: `google.com` or `recaptcha.net`.
    /// Default value: `google.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) api_domain: Option<Cow<'a, str>>,
}
