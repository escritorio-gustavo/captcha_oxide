use std::borrow::Cow;

use url::Url;

use crate::{
    captcha_types::geetest::{
        type_state::{ChallengeMissing, GtMissing},
        GeetestTypes,
    },
    type_state::UrlMissing,
    CaptchaTask,
};

use super::{builder::GeeTestV3Builder, solution::GeeTestV3Solution};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeeTestV3<'a> {
    #[serde(flatten)]
    pub(super) task_type: GeetestTypes<'a>,

    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) gt: Cow<'a, str>,
    pub(super) challenge: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) geetest_api_server_subdomain: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,

    pub(super) version: u8,
}

impl<'a> CaptchaTask for GeeTestV3<'a> {
    type Solution = GeeTestV3Solution<'a>;
    type Builder = GeeTestV3Builder<'a, UrlMissing, GtMissing, ChallengeMissing>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(20)
    }
}
