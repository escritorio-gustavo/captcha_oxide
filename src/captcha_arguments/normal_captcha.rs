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

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::env;

    use super::NormalCaptcha;
    use crate::{
        captcha_arguments::normal_captcha::NormalCaptchaMethods, response::RequestContent,
        solver::CaptchaSolver,
    };

    #[tokio::test]
    #[ignore = "These tests should run all at once, as this will likely cause a 429 block from the 2captcha API"]
    async fn normal_captcha() {
        dotenv().unwrap();
        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let args = NormalCaptcha {
            method: NormalCaptchaMethods::Base64("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".into()),
            phrase: Some(true),
            case_sensitive: Some(true),
            ..Default::default()
        };

        let solution = solver.solve(args).await;

        assert!(solution.is_ok());

        let solution = solution.unwrap().solution;
        match solution {
            RequestContent::String(solution) => {
                assert_eq!(solution.to_lowercase().replace(' ', ""), "w68hp");
            }
            _ => unreachable!("Wrong enum variant"),
        }
    }
}
