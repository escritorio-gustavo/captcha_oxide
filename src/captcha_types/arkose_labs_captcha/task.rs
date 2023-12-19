use std::{borrow::Cow, marker::PhantomData};

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::{captcha_types::empty_data::Empty, CaptchaTask};

/// Represents the data required by the 2captcha API to solve a
/// ArkoseLabsCaptcha challenge
///
/// # Example
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     captcha_types::arkose_labs_captcha::ArkoseLabsCaptcha,
/// };
///
/// # fn main() -> Result<(), captcha_oxide::Error> {
/// let captcha = <ArkoseLabsCaptcha>::builder()
///     .website_url("https://www.example.com")
///     .website_public_key("6220FF23-9856-3A6F-9FF1-A14F88123F55")
///     .build()?;
/// # Ok(())
/// # }
/// ```
///
/// The angle brackets (`<>`) around [`ArkoseLabsCaptcha`] allow the use of the
/// default type provided to the generic argument, so you don't need to
/// create a serializable unit struct if you don't plan to use the
/// [`ArkoseLabsCaptcha::data`] field
#[proxy_task(with_proxy = "FunCaptchaTask", proxyless = "FunCaptchaTaskProxyless", crate = crate)]
#[derive(CaptchaTask, serde::Serialize)]
#[task(timeout = 20, crate = crate, solution = super::solution::ArkoseLabsCaptchaSolution<'a>)]
#[serde(rename_all = "camelCase")]
pub struct ArkoseLabsCaptcha<'a, T = Empty>
where
    T: serde::Serialize,
{
    #[serde(skip)]
    #[task(default = Default::default)]
    pub(super) _x: PhantomData<T>,

    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse })})]
    pub(super) website_url: Url,

    /// ArkoseLabsCaptcha public key. The public key can be found in
    /// the value of the `data-pkey` parameter of the FunCaptcha `div` element,
    /// or you can find an element with `name=fc-token` and from its value cut
    /// out the key that is specified after `pk`.
    pub(super) website_public_key: Cow<'a, str>,

    /// Custom subdomain used to load the captcha widget, e.g.: `sample-api.arkoselabs.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) funcaptcha_api_jssubdomain: Option<Cow<'a, str>>,

    /// Additional data payload object.
    /// This data will be converted to a JSON string internally
    #[serde(skip_serializing_if = "Option::is_none")]
    #[task(builder_type = Option<T>, parse_with = { fallible({ path = serde_json::to_string, parse_ref }) })]
    pub(super) data: Option<Cow<'a, str>>,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,
}
