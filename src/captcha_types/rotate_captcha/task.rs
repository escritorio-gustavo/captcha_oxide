use std::borrow::Cow;

use captcha_oxide_derive::CaptchaTask;

/// This method is used to solve captchas where you need to rotate an object
/// to place it properly. Returns the required rotation angle.
///
/// # Example
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     captcha_types::rotate_captcha::RotateCaptcha,
/// };
///
/// let captcha = RotateCaptcha::builder()
///     .body("R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==")
///     .comment(Some("Position the image properly"))
///     .angle(Some(60_u16))
///     .build();
/// ```
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 5, solution = super::solution::RotateCaptchaSolution, crate = crate)]
#[serde(rename_all = "camelCase", tag = "type", rename = "RotateTask")]
pub struct RotateCaptcha<'a> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    pub(super) body: Cow<'a, str>,

    /// One step rotation angle. You can count how many steps are required
    /// to rotate the image 360 degrees and then divide 360 by this count
    /// to get the angle value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) angle: Option<u16>,

    /// A comment will be shown to the workers to help them solve the captcha properly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) comment: Option<Cow<'a, str>>,

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) img_instructions: Option<Cow<'a, str>>,
}
