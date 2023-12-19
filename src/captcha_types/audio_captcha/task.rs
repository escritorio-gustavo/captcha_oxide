use std::borrow::Cow;

use crate::CaptchaTask;

/// Represents the data required by the 2captcha API to solve a
/// AudioCaptcha challenge
///
/// # Example
/// ```
/// use captcha_oxide::{
///     CaptchaTask,
///     captcha_types::audio_captcha::{AudioCaptcha, Language},
/// };
///
/// let captcha = AudioCaptcha::builder()
///     .body("R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==")
///     .language(Language::Portuguese)
///     .build();
/// ```
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 5, solution = super::solution::AudioCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase", tag = "type", rename = "AudioTask")]
pub struct AudioCaptcha<'a> {
    /// Base64 encoded audio file in mp3 format
    pub(super) body: Cow<'a, str>,

    /// The language of the audio recording.
    /// Supported languages are:
    /// * `Portuguese`
    /// * `English`
    /// * `French`
    /// * `German`
    /// * `Greek`
    /// * `Russian`
    #[serde(rename = "lang")]
    pub(super) language: Language,
}

#[derive(Default, serde::Serialize, Debug)]
pub enum Language {
    #[serde(rename = "en")]
    #[default]
    English,

    #[serde(rename = "pt")]
    Portuguese,

    #[serde(rename = "fr")]
    French,

    #[serde(rename = "de")]
    German,

    #[serde(rename = "el")]
    Greek,

    #[serde(rename = "ru")]
    Russian,
}
