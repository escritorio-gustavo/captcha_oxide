use std::borrow::Cow;

use captcha_oxide_derive::CaptchaTask;

#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 5, solution = super::solution::RotateCaptchaSolution, crate = crate)]
#[serde(rename_all = "camelCase", tag = "type", rename = "RotateTask")]
pub struct RotateCaptcha<'a> {
    pub(super) body: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) angle: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) comment: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) img_instructions: Option<Cow<'a, str>>,
}
