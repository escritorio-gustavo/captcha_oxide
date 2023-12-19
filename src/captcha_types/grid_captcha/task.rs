use std::borrow::Cow;

use crate::CaptchaTask;

use super::type_state::*;

/// This method can be used to bypass tasks where a grid is applied to an
/// image and you need to click on grid tiles, like reCAPTCHA or hCaptcha images.
/// 
/// # Example
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     captcha_types::grid_captcha::GridCaptcha,
/// };
///
/// let captcha = GridCaptcha::builder()
///     .body("/9j/4AAQSkZJ...OGSRF//Z")
///     .comment("Select all vehicles")
///     .rows(Some(3))
///     .columns(Some(3))
///     .build();
/// ```
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase", tag = "type", rename = "GridTask")]
pub struct GridCaptcha<'a> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    pub(super) body: Cow<'a, str>,

    /// Number of grid rows
    pub(super) rows: Option<u8>,

    /// Number of grid columns
    pub(super) columns: Option<u8>,

    /// A comment will be shown to the workers to help them solve the captcha properly
    pub(super) comment: Option<Cow<'a, str>>,

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    pub(super) img_instructions: Option<Cow<'a, str>>,
}

impl<'a> CaptchaTask for GridCaptcha<'a> {
    type Solution = super::solution::GridCaptchaSolution;
    type Builder =
        super::builder::GridCaptchaBuilder<BodyMissing, CommentMissing, ImgInstructionsMissing>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(5)
    }
}
