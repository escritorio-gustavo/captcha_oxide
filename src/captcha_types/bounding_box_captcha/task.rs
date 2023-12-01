use std::borrow::Cow;

use crate::CaptchaTask;

#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 5, solution_has_lifetime = false)]
#[serde(rename_all = "camelCase", tag = "type", rename = "DrawAroundTask")]
pub struct BoundingBoxCaptcha<'a> {
    pub(super) body: Cow<'a, str>,
    pub(super) comment: Option<Cow<'a, str>>,
    pub(super) img_instructions: Option<Cow<'a, str>>,
}
