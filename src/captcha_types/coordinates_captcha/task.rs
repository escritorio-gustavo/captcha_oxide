use std::borrow::Cow;

use captcha_oxide_derive::CaptchaTask;

#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 5, solution_has_lifetime = false)]
#[serde(rename_all = "camelCase", tag = "type", rename = "CoordinatesTask")]
pub struct CoordinatesCaptcha<'a> {
    pub(super) body: Cow<'a, str>,
    pub(super) comment: Option<Cow<'a, str>>,
    pub(super) img_instructions: Option<Cow<'a, str>>,
}
