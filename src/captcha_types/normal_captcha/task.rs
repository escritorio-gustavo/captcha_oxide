use std::borrow::Cow;

use crate::CaptchaTask;

/// Normal CAPTCHA is an image that contains distored but human-readable text.
/// To solve the captcha, you have to type the text from the image into a form.
///
/// # Example
/// ```
/// use captcha_oxide::{CaptchaTask, captcha_types::normal_captcha::NormalCaptcha};
///
/// let captcha = NormalCaptcha::builder()
///     .body("R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==")
///     .comment(Some("Enter the text you see on the image"))
///     .build();
/// ```
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 5, solution = super::solution::NormalCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase", tag = "type", rename = "ImageToTextTask")]
pub struct NormalCaptcha<'a> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    pub(super) body: Cow<'a, str>,

    /// Indicates if the image contains words separated by space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) phrase: Option<bool>,

    /// Indicates if the image is case sensitive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) case: Option<bool>,

    /// What types of characters are allowed in the answer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) numeric: Option<AnswerType>,

    /// Indicates if the image contains a calculation, such as `2 + 2`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) math: Option<bool>,

    /// Minimum length of the answer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) min_length: Option<u32>,

    /// Maximum length of the answer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) max_length: Option<u32>,

    /// A comment will be shown to the workers to help them solve the captcha properly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) comment: Option<Cow<'a, str>>,

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) img_instructions: Option<Cow<'a, str>>,
}

#[derive(serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum AnswerType {
    NoPreference = 0,
    Numeric = 1,
    Alphabetical = 2,
    AlphabeticalOrNumerical = 3,
    AlphaNumerical = 4,
}
