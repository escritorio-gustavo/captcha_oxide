use std::borrow::Cow;

use captcha_oxide_derive::CaptchaTask;

#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 5, solution = super::solution::CoordinatesCaptchaSolution, crate = crate)]
#[serde(rename_all = "camelCase", tag = "type", rename = "CoordinatesTask")]
pub struct CoordinatesCaptcha<'a> {
    pub(super) body: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) comment: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) img_instructions: Option<Cow<'a, str>>,
}
