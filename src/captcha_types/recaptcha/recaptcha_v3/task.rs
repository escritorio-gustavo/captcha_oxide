use std::borrow::Cow;
use url::Url;

use crate::{
    captcha_types::recaptcha::{
        solution::ReCaptchaSolution,
        type_state::{NoMinScoreProvided, NoUrlProvided, NoWebsiteKeyProvided},
    },
    CaptchaTask,
};

use super::RecaptchaV3Builder;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaV3<'a> {
    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,
    pub(super) min_score: f32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) page_action: Option<Cow<'a, str>>,
    pub(super) is_enterprise: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) api_domain: Option<Cow<'a, str>>,
}

impl<'a> CaptchaTask for RecaptchaV3<'a> {
    type Solution = ReCaptchaSolution<'a>;
    type Builder = RecaptchaV3Builder<'a, NoUrlProvided, NoWebsiteKeyProvided, NoMinScoreProvided>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
