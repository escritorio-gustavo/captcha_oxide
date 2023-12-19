use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::{
    captcha_types::{empty_data::Empty, geetest::geetest_v4::type_state::*},
    CaptchaTask,
};

use super::{builder::GeeTestV4Builder, solution::GeeTestV4Solution};

/// Represents the data required by the 2captcha API to solve a
/// GeeTestV4 challenge
///
/// # Example
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     captcha_types::geetest::GeeTestV4
/// };
///
/// # fn main() -> Result<(), captcha_oxide::Error> {
/// let captcha = <GeeTestV4>::builder()
///     .website_url("https://someurl.com/latest")
///     .gt("81388ea1fc187e0c335c0a8907ff2625")
///     .challenge("2e2f0f65240058b683cb6ea21c303eea6n")
///     .captcha_id("e392e1d7fd421dc63325744d5a2b9c73")
///     .build()?;
/// # Ok(())
/// # }
/// ```
///
/// The angle brackets (`<>`) around [`GeeTestV4`] allow the use of the
/// default type provided to the generic argument, so you don't need to
/// create a serializable unit struct if you don't plan to use the
/// [`GeetestV4::init_params`] field
#[proxy_task(with_proxy = "GeeTestTask", proxyless = "GeeTestTaskProxyless")]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeeTestV4<'a, T = Empty>
where
    T: serde::Serialize,
{
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
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

    /// Captcha parameters passed to `initGeetest`
    pub(super) init_parameters: InitParameters<'a, T>,

    /// Internally set to `4`
    pub(super) version: u8,
}

#[derive(Debug, serde::Serialize)]
pub struct InitParameters<'a, T> {
    pub(super) captcha_id: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub(super) data: Option<T>,
}

impl<'a, T> CaptchaTask for GeeTestV4<'a, T>
where
    T: serde::Serialize,
{
    type Solution = GeeTestV4Solution<'a>;
    type Builder =
        GeeTestV4Builder<'a, UrlMissing, GtMissing, ChallengeMissing, CaptchaIdMissing, T>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
