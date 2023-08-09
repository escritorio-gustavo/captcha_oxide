pub use super::type_state::{
    img_instructions::{ImgInstructions, ImgInstructionsNotProvided},
    method::{Method, MethodNotProvided},
    text_instructions::{TextInstructions, TextInstructionsNotProvided},
};

use crate::arguments::{
    character_restrictions::CharacterRestrictions,
    language::Language,
    normal_captcha::{ImageInstructions, NormalCaptcha, NormalCaptchaMethods},
};

#[derive(Default, Debug, Clone)]
/// Builds a [`NormalCaptcha`] instance using the typestate pattern
/// to help avoid sending avoid inconsistent data to the
/// 2captcha API
///
/// # Example
/// ```
/// use captcha_oxide::arguments::{CaptchaArguments, NormalCaptcha};
///
/// let args = NormalCaptcha::builder()
///     .method(NormalCaptchaMethods::Base64("BASE_64_IMAGE".into()))
///     .text_instructions("Repeat the characters in the image")
///     .build();
/// ```
pub struct NormalCaptchaBuilder<T, U, V> {
    method: T,
    text_instructions: U,
    image_instructions: V,
    numeric: Option<CharacterRestrictions>,
    pingback: Option<String>,
    case_sensitive: Option<bool>,
    requires_calculation: Option<bool>,
    phrase: Option<bool>,
    min_len: Option<u8>,
    max_len: Option<u8>,
    language: Option<Language>,
    language_code: Option<String>,
}

impl
    NormalCaptchaBuilder<MethodNotProvided, TextInstructionsNotProvided, ImgInstructionsNotProvided>
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl NormalCaptchaBuilder<Method, TextInstructions, ImgInstructions> {
    pub fn build(self) -> NormalCaptcha {
        NormalCaptcha {
            method: self.method.0,
            text_instructions: Some(self.text_instructions.0),
            image_instructions: Some(self.image_instructions.0),
            numeric: self.numeric,
            pingback: self.pingback,
            case_sensitive: self.case_sensitive,
            requires_calculation: self.requires_calculation,
            phrase: self.phrase,
            min_len: self.min_len,
            max_len: self.max_len,
            language: self.language,
            language_code: self.language_code,
        }
    }
}

impl NormalCaptchaBuilder<Method, TextInstructionsNotProvided, ImgInstructions> {
    pub fn build(self) -> NormalCaptcha {
        NormalCaptcha {
            method: self.method.0,
            text_instructions: None,
            image_instructions: Some(self.image_instructions.0),
            numeric: self.numeric,
            pingback: self.pingback,
            case_sensitive: self.case_sensitive,
            requires_calculation: self.requires_calculation,
            phrase: self.phrase,
            min_len: self.min_len,
            max_len: self.max_len,
            language: self.language,
            language_code: self.language_code,
        }
    }
}

impl NormalCaptchaBuilder<Method, TextInstructions, ImgInstructionsNotProvided> {
    pub fn build(self) -> NormalCaptcha {
        NormalCaptcha {
            method: self.method.0,
            text_instructions: Some(self.text_instructions.0),
            image_instructions: None,
            numeric: self.numeric,
            pingback: self.pingback,
            case_sensitive: self.case_sensitive,
            requires_calculation: self.requires_calculation,
            phrase: self.phrase,
            min_len: self.min_len,
            max_len: self.max_len,
            language: self.language,
            language_code: self.language_code,
        }
    }
}

impl<T, U, V> NormalCaptchaBuilder<T, U, V> {
    /// The way you intend on sending the captcha image:
    /// * As multipart/form-data
    /// * Or as a base64 string
    pub fn method(self, method: NormalCaptchaMethods) -> NormalCaptchaBuilder<Method, U, V> {
        NormalCaptchaBuilder {
            method: Method(method),
            text_instructions: self.text_instructions,
            image_instructions: self.image_instructions,
            numeric: self.numeric,
            pingback: self.pingback,
            case_sensitive: self.case_sensitive,
            requires_calculation: self.requires_calculation,
            phrase: self.phrase,
            min_len: self.min_len,
            max_len: self.max_len,
            language: self.language,
            language_code: self.language_code,
        }
    }

    /// Text will be shown to worker to help him to solve the captcha correctly.
    /// For example: type red symbols only.
    ///
    /// This is required if you don't send `image_instructions`
    pub fn text_instructions(
        self,
        text_instructions: impl Into<String>,
    ) -> NormalCaptchaBuilder<T, TextInstructions, V> {
        NormalCaptchaBuilder {
            method: self.method,
            text_instructions: TextInstructions(text_instructions.into()),
            image_instructions: self.image_instructions,
            numeric: self.numeric,
            pingback: self.pingback,
            case_sensitive: self.case_sensitive,
            requires_calculation: self.requires_calculation,
            phrase: self.phrase,
            min_len: self.min_len,
            max_len: self.max_len,
            language: self.language,
            language_code: self.language_code,
        }
    }

    /// Image with instruction for solving the captcha
    ///
    /// This is required if you don't send `text_instructions`
    pub fn image_instructions(
        self,
        image_instructions: ImageInstructions,
    ) -> NormalCaptchaBuilder<T, U, ImgInstructions> {
        NormalCaptchaBuilder {
            method: self.method,
            text_instructions: self.text_instructions,
            image_instructions: ImgInstructions(image_instructions),
            numeric: self.numeric,
            pingback: self.pingback,
            case_sensitive: self.case_sensitive,
            requires_calculation: self.requires_calculation,
            phrase: self.phrase,
            min_len: self.min_len,
            max_len: self.max_len,
            language: self.language,
            language_code: self.language_code,
        }
    }

    pub fn numeric(mut self, numeric: CharacterRestrictions) -> Self {
        self.numeric = Some(numeric);
        self
    }

    pub fn pingback(mut self, pingback: impl Into<String>) -> Self {
        self.pingback = Some(pingback.into());
        self
    }

    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = Some(case_sensitive);
        self
    }

    pub fn requires_calculation(mut self, requires_calculation: bool) -> Self {
        self.requires_calculation = Some(requires_calculation);
        self
    }

    pub fn phrase(mut self, phrase: bool) -> Self {
        self.phrase = Some(phrase);
        self
    }

    pub fn min_len(mut self, min_len: u8) -> Self {
        self.min_len = Some(min_len);
        self
    }

    pub fn max_len(mut self, max_len: u8) -> Self {
        self.max_len = Some(max_len);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
        self.language_code = Some(language_code.into());
        self
    }
}
