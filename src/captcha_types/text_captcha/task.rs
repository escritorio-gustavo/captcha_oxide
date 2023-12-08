use std::borrow::Cow;

use crate::CaptchaTask;

#[derive(serde::Serialize, CaptchaTask)]
#[serde(tag = "type", rename = "TextCaptchaTask")]
#[task(timeout = 5, solution = super::solution::TextCaptchaSolution<'a>, crate = crate)]
pub struct TextCaptcha<'a> {
    pub(super) comment: Cow<'a, str>,
}
