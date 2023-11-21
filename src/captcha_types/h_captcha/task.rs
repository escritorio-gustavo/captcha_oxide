use captcha_oxide_derive::proxy_task;
use std::borrow::Cow;
use url::Url;

use crate::{
    captcha_types::{
        empty_data::Empty,
        h_captcha::{builder::HCaptchaBuilder, solution::HCaptchaSolution},
        CaptchaTask,
    },
    type_state::{UrlMissing, WebsiteKeyMissing},
};

#[proxy_task(with_proxy = "HCaptchaTask", proxyless = "HCaptchaTaskProxyless")]
#[serde(rename_all = "camelCase")]
/// Represents the data required by the 2captcha API to solve a
/// HCaptcha challenge
///
/// # Note
/// If you need to use this struct but there is no `enterprise_payload`
/// to be sent, you should invoke the builder using the following syntax:
/// ```
/// use captcha_oxide::captcha_types::{CaptchaTask, HCaptcha};
/// use url::Url;
///
/// # fn main() -> Result<(), captcha_oxide::Error> {
/// let captcha = <HCaptcha>::builder()
///     .website_url(Url::parse("http://someurl.com")?)
///     .website_key("SOME_KEY")
///     .build();
/// # Ok(())
/// # }
/// ```
///
/// The angle brackets (`<>`) around [`HCaptcha`] allow the use of the
/// default type provided to the generic argument, so you don't need to
/// create a serializable unit struct
pub struct HCaptcha<'a, T = Empty>
where
    T: serde::Serialize,
{
    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,
    pub(super) is_invisible: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) enterprise_payload: Option<T>,
}

impl<'a, T> CaptchaTask for HCaptcha<'a, T>
where
    T: serde::Serialize,
{
    type Solution = HCaptchaSolution<'a>;
    type Builder = HCaptchaBuilder<'a, UrlMissing, WebsiteKeyMissing, T>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
