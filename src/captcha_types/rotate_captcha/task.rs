use std::borrow::Cow;

use captcha_oxide_derive::CaptchaTask;

#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 5, solution_has_lifetime = false)]
#[serde(rename_all = "camelCase", tag = "type", rename = "RotateTask")]
pub struct RotateCaptcha<'a> {
    pub(super) body: Cow<'a, str>,
    pub(super) angle: Option<u16>,
    pub(super) comment: Option<Cow<'a, str>>,
    pub(super) img_instructions: Option<Cow<'a, str>>,
}
