use std::borrow::Cow;

use captcha_oxide_derive::CaptchaTask;

/// This method can be used to bypass tasks where you need to click
/// on some points of an image. \
/// It can be also used for cases where you need to calculate a distance
/// between points. \
/// \
/// For example, to bypass custom slider captchas you can instruct our worker
/// to click on a particular point of the image using the
/// [`CoordinatesCaptcha::comment`] and [`CoordinatesCaptcha::img_instructions`]
/// parameters and then use the point coordinates to calculate the distance
/// between the slider's start and end points and move the slider.
///
/// # Example
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     captcha_types::coordinates_captcha::CoordinatesCaptcha,
/// };
///
/// let captcha = CoordinatesCaptcha::builder()
///     .body("/9j/4AAQSkZJRgABAQAAAQ..HIAAAAAAQwAABtbnRyUkdCIFhZ.wc5GOGSRF//Z")
///     .comment(Some("Click the green apple"))
///     .build();
/// ```
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 5, solution = super::solution::CoordinatesCaptchaSolution, crate = crate)]
#[serde(rename_all = "camelCase", tag = "type", rename = "CoordinatesTask")]
pub struct CoordinatesCaptcha<'a> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    pub(super) body: Cow<'a, str>,

    /// A comment will be shown to the workers to help them solve the captcha properly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) comment: Option<Cow<'a, str>>,

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) img_instructions: Option<Cow<'a, str>>,
}
