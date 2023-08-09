use crate::arguments::{language::Language, TextCaptcha};

pub use super::type_state::{CaptchaText, CaptchaTextNotProvided};

#[derive(Default, Debug, Clone)]
/// Builds a [`TextCaptcha`] instance using the typestate pattern
/// to help avoid sending avoid inconsistent data to the
/// 2captcha API
///
/// # Example
/// ```
/// use captcha_oxide::arguments::TextCaptcha;
///
/// let args = TextCaptcha::builder()
///     .text_captcha("What is 2 + 2?")
///     .build();
/// ```
pub struct TextCaptchaBuilder<T> {
    text_captcha: T,
    language_code: Option<String>,
    language: Option<Language>,
    pingback: Option<String>,
}

impl TextCaptchaBuilder<CaptchaTextNotProvided> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TextCaptchaBuilder<CaptchaText> {
    pub fn build(self) -> TextCaptcha {
        TextCaptcha {
            text_captcha: self.text_captcha.0,
            language_code: self.language_code,
            language: self.language,
            pingback: self.pingback,
        }
    }
}

impl<T> TextCaptchaBuilder<T> {
    /// Text that will be shown to the worker to help them solve the captcha correctly.
    /// For example: type red symbols only.
    pub fn text_captcha(self, text_captcha: impl Into<String>) -> TextCaptchaBuilder<CaptchaText> {
        TextCaptchaBuilder {
            text_captcha: CaptchaText(text_captcha.into()),
            language: self.language,
            language_code: self.language_code,
            pingback: self.pingback,
        }
    }

    /// Language code, such as `pt`, `en`, `es`
    pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
        self.language_code = Some(language_code.into());
        self
    }

    /// The alphabet to be used
    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    /// Callback URL where you wish to receive the response
    pub fn pingback(mut self, pingback: impl Into<String>) -> Self {
        self.pingback = Some(pingback.into());
        self
    }
}
