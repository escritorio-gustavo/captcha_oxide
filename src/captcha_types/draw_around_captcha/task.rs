use std::borrow::Cow;

use super::type_state::{BodyMissing, CommentMissing, ImgInstructionsMissing};
use crate::CaptchaTask;

/// This method can be used to bypass tasks where you need to draw
/// a line around a specific object shown on an image.
///
/// # Example
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     captcha_types::draw_around_captcha::DrawAroundCaptcha,
/// };
///
/// let captcha = DrawAroundCaptcha::builder()
///     .body("/9j/4AAQSkZJ...OGSRF//Z")
///     .comment("Draw around an apple")
///     .build();
/// ```
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase", tag = "type", rename = "DrawAroundTask")]
pub struct DrawAroundCaptcha<'a> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    pub(super) body: Cow<'a, str>,

    /// A comment will be shown to the workers to help them solve the captcha properly
    /// The [`DrawAroundCaptcha::comment`] property is required if
    /// [`DrawAroundCaptcha::img_instructions`] is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) comment: Option<Cow<'a, str>>,

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    /// The [`DrawAroundCaptcha::img_instructions`] property is required if
    /// the [`DrawAroundCaptcha::comment`] property is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) img_instructions: Option<Cow<'a, str>>,
}

impl<'a> CaptchaTask for DrawAroundCaptcha<'a> {
    type Solution = super::solution::DrawAroundCaptchaSolution;
    type Builder = super::builder::DrawAroundCaptchaBuilder<
        BodyMissing,
        CommentMissing,
        ImgInstructionsMissing,
    >;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(5)
    }
}
