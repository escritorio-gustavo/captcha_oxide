use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};

use crate::{error::Error, TWO_CAPTCHA_DEVELOPER_ID};

use super::{
    arguments::CaptchaArguments, character_restrictions::CharacterRestrictions, language::Language,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct NormalCaptcha {
    pub method: NormalCaptchaMethods,
    pub numeric: Option<CharacterRestrictions>,
    pub pingback: Option<String>,

    /// Whether or not the captcha is case-sensitive
    pub case_sensitive: Option<bool>,

    /// Whether or not the captcha requires a calculation such as `5 + 3`
    pub requires_calculation: Option<bool>,

    /// Whether or not the captcha contains more than one word
    pub phrase: Option<bool>,

    /// Text will be shown to worker to help him to solve the captcha correctly.
    /// For example: type red symbols only.
    pub text_instructions: String,

    /// Must be in range (1..=20)
    pub min_len: Option<u8>,

    /// Must be in range (1..=20)
    pub max_len: Option<u8>,

    pub language: Option<Language>,

    /// Language code such as `pt-BR`
    pub language_code: Option<String>,
}

impl CaptchaArguments for NormalCaptcha {
    fn to_request_params(&self, api_key: String) -> Result<Form, Error> {
        let mut request_body = Form::new()
            .text("key", api_key)
            .text("json", "1")
            .text("header_acao", "1")
            .text("soft_id", TWO_CAPTCHA_DEVELOPER_ID)
            .text("textinstructions", self.text_instructions.clone());

        match &self.method {
            NormalCaptchaMethods::Base64(data) => {
                if data.is_empty() {
                    panic!(
                        "The data in this enum variant must not be empty. \
                        Make sure you don't use the `Default` trait to fill the `method` field of the \
                        `NormalCaptcha` struct"
                    );
                }

                request_body = request_body
                    .text("method", "base64")
                    .text("body", data.clone());
            }
            NormalCaptchaMethods::Post {
                bytes,
                mime_str,
                file_extension,
            } => {
                let part = Part::bytes((*bytes).clone())
                    .file_name(format!("captcha.{}", (*file_extension).replace('.', "")))
                    .mime_str(mime_str)
                    .map_err(|_| Error::FileParseError)?;

                request_body = request_body.text("method", "post").part("file", part);
            }
        }

        if let Some(calc) = &self.requires_calculation {
            request_body = request_body.text("calc", if *calc { "1" } else { "0" });
        }

        if let Some(phrase) = &self.phrase {
            request_body = request_body.text("phrase", if *phrase { "1" } else { "0" });
        }

        if let Some(case_sensitive) = &self.case_sensitive {
            request_body = request_body.text("regsense", if *case_sensitive { "1" } else { "0" });
        }

        if let Some(lang) = &self.language_code {
            request_body = request_body.text("lang", lang.clone());
        }

        if let Some(pingback) = &self.pingback {
            request_body = request_body.text("pingback", pingback.clone());
        }

        if let Some(min_len) = &self.min_len {
            request_body = request_body.text("min_len", (*min_len).to_string());
        }

        if let Some(max_len) = &self.max_len {
            request_body = request_body.text("max_len", (*max_len).to_string());
        }

        if let Some(language) = &self.language {
            request_body = request_body.text(
                "language",
                match *language {
                    Language::NotSpecified => "0",
                    Language::Cyrillic => "1",
                    Language::Latin => "2",
                },
            );
        }

        if let Some(numeric) = &self.numeric {
            request_body = request_body.text(
                "numeric",
                match *numeric {
                    CharacterRestrictions::NotSpecified => "0",
                    CharacterRestrictions::OnlyNumbers => "1",
                    CharacterRestrictions::OnlyLetters => "2",
                    CharacterRestrictions::OnlyNumbersOrOnlyLetters => "3",
                    CharacterRestrictions::BothNumbersAndLetters => "4",
                },
            );
        }

        Ok(request_body)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NormalCaptchaMethods {
    Post {
        bytes: Vec<u8>,
        mime_str: String,
        file_extension: String,
    },
    Base64(String),
}

impl Default for NormalCaptchaMethods {
    fn default() -> Self {
        Self::Base64("".into())
    }
}
