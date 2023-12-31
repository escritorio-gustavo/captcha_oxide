use std::borrow::Cow;

use super::type_state::{BodyMissing, CommentMissing, ImgInstructionsMissing};
use crate::CaptchaTask;

/// Can be used to solve tasks where you need to select a specific
/// object or draw a box around an object shown on an image.
///
/// # Example
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     captcha_types::bounding_box_captcha::BoundingBoxCaptcha
/// };
///
/// let captcha = BoundingBoxCaptcha::builder()
///     .body("/9j/4AAQSkZJRgABAQAAAQ..HIAAAAAAQwAABtbnRyUkdCIFhZ.wc5GOGSRF//Z")
///     .comment("Draw a box around the car")
///     .build();
/// ```
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase", tag = "type", rename = "DrawAroundTask")]
pub struct BoundingBoxCaptcha<'a> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    pub(super) body: Cow<'a, str>,

    /// A comment will be shown to workers to help them solve the captcha properly.
    /// The [`BoundingBoxCaptcha::comment`] property is required if
    /// [`BoundingBoxCaptcha::img_instructions`] is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) comment: Option<Cow<'a, str>>,

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    /// The [`BoundingBoxCaptcha::img_instructions`] property is required if
    /// the [`BoundingBoxCaptcha::comment`] property is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) img_instructions: Option<Cow<'a, str>>,
}

impl<'a> CaptchaTask for BoundingBoxCaptcha<'a> {
    type Solution = super::solution::BoundingBoxCaptchaSolution;
    type Builder = super::builder::BoundingBoxCaptchaBuilder<
        BodyMissing,
        CommentMissing,
        ImgInstructionsMissing,
    >;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(5)
    }
}
