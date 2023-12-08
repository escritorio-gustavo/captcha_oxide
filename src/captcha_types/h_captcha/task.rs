use captcha_oxide_derive::proxy_task;
use std::borrow::Cow;
use url::Url;

use crate::captcha_types::{empty_data::Empty, CaptchaTask};

/// Represents the data required by the 2captcha API to solve a
/// HCaptcha challenge
///
/// # Example
/// ```
/// use captcha_oxide::{CaptchaTask, captcha_types::h_captcha::HCaptcha};
///
/// # fn main() -> Result<(), captcha_oxide::Error> {
/// let captcha = <HCaptcha>::builder()
///     .website_url("http://someurl.com")
///     .website_key("SOME_KEY")
///     .build()?;
/// # Ok(())
/// # }
/// ```
///
/// The angle brackets (`<>`) around [`HCaptcha`] allow the use of the
/// default type provided to the generic argument, so you don't need to
/// create a serializable unit struct if you don't plan to use the
/// `enterprise_payload` field
#[proxy_task(with_proxy = "HCaptchaTask", proxyless = "HCaptchaTaskProxyless", crate = crate)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::solution::HCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct HCaptcha<'a, T = Empty>
where
    T: serde::Serialize,
{
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) is_invisible: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) enterprise_payload: Option<T>,
}
