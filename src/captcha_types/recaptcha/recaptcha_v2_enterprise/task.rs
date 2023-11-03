use std::borrow::Cow;
use url::Url;

use crate::{
    captcha_types::{
        empty_data::Empty,
        recaptcha::{
            recaptcha_v2_enterprise::builder::RecaptchaV2EnterpriseBuilder,
            solution::ReCaptchaSolution,
        },
        type_state::{NoUrlProvided, NoWebsiteKeyProvided},
        CaptchaTask,
    },
    proxy::Proxy,
};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
/// Represents the data required by the 2captcha API to solve a
/// reCaptcha V2 Enterprise challenge
///
/// # Note
/// If you need to use this struct but there is no `enterprise_payload`
/// to be sent, you should invoke the builder using the following syntax:
/// ```
/// let captcha = <RecaptchaV2Enterprise>::builder()
///     .website_url(url::Url::from_str("http://someurl.com"))
///     .website_key("SOME_KEY")
///     .build();
/// ```
///
/// The angle brackets (`<>`) around [`RecaptchaV2Enterprise`] allow the
/// use of the default type provided to the generic argument, so you don't
/// need to create a serializable unit struct
pub struct RecaptchaV2Enterprise<'a, T = Empty>
where
    T: serde::Serialize,
{
    #[serde(flatten)]
    pub(super) task_type: RecaptchaV2EnterpriseTypes<'a>,

    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) enterprise_payload: Option<T>,

    pub(super) is_invisible: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) cookies: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) api_domain: Option<Cow<'a, str>>,
}

#[derive(serde::Serialize)]
#[serde(tag = "type")]
pub enum RecaptchaV2EnterpriseTypes<'a> {
    #[serde(rename = "RecaptchaV2EnterpriseTaskProxyless")]
    ProxyLess,

    #[serde(rename = "RecaptchaV2EnterpriseTask")]
    WithProxy(Proxy<'a>),
}

impl<'a, T> CaptchaTask for RecaptchaV2Enterprise<'a, T>
where
    T: serde::Serialize,
{
    type Solution = ReCaptchaSolution<'a>;
    type Builder = RecaptchaV2EnterpriseBuilder<'a, NoUrlProvided, NoWebsiteKeyProvided, T>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
