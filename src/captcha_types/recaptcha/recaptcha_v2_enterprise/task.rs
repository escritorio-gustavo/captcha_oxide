use captcha_oxide_derive::proxy_task;
use std::borrow::Cow;
use url::Url;

use crate::{captcha_types::empty_data::Empty, CaptchaTask};

#[proxy_task(
    with_proxy = "RecaptchaV2EnterpriseTask",
    proxyless = "RecaptchaV2EnterpriseTaskProxyless",
    crate = crate,
)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::super::solution::ReCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
/// Represents the data required by the 2captcha API to solve a
/// reCaptcha V2 Enterprise challenge
///
/// # Example
/// ```
/// use captcha_oxide::{
///     Error,
///     CaptchaTask,
///     captcha_types::recaptcha::RecaptchaV2Enterprise
/// };
///
/// # fn main() -> Result<(), Error> {
/// let captcha = <RecaptchaV2Enterprise>::builder()
///     .website_url("http://someurl.com")
///     .website_key("SOME_KEY")
///     .build()?;
/// # Ok(())
/// # }
/// ```
///
/// The angle brackets (`<>`) around [`RecaptchaV2Enterprise`] allow the
/// use of the default type provided to the generic argument, so you don't
/// need to create a serializable unit struct if you don't plan to use the
/// `ènterprise_payload` field
pub struct RecaptchaV2Enterprise<'a, T = Empty>
where
    T: serde::Serialize,
{
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) enterprise_payload: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) is_invisible: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[task(builder_type = Option<crate::cookie::Cookies<'a>>, parse_with = { infallible({ path = crate::cookie::Cookies::stringify, parse_ref }) })]
    pub(super) cookies: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) api_domain: Option<Cow<'a, str>>,
}
