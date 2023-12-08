use std::borrow::Cow;

use crate::CaptchaTask;

#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 5, solution = super::solution::DrawAroundCaptchaSolution, crate = crate)]
#[serde(rename_all = "camelCase", tag = "type", rename = "DrawAroundTask")]
pub struct DrawAroundCaptcha<'a> {
    pub(super) body: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) comment: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) img_instructions: Option<Cow<'a, str>>,
}
