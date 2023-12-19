use std::borrow::Cow;

use crate::CaptchaTask;

/// # Example
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     captcha_types::text_captcha::TextCaptcha,
/// };
///
/// let captcha = TextCaptcha::builder()
///     .comment("If tomorrow is Saturday, what day is today?")
///     .build();
/// ```
#[derive(serde::Serialize, CaptchaTask)]
#[serde(tag = "type", rename = "TextCaptchaTask")]
#[task(timeout = 5, solution = super::solution::TextCaptchaSolution<'a>, crate = crate)]
pub struct TextCaptcha<'a> {
    /// Text with a question you need to answer.
    pub(super) comment: Cow<'a, str>,
}
