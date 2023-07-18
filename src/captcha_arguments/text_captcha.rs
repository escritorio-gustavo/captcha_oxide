use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::{error::Error, TWO_CAPTCHA_DEVELOPER_ID};

use super::{arguments::CaptchaArguments, language::Language};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TextCaptcha {
    pub language: Option<Language>,
    /// Language code
    pub lang: Option<String>,
    /// Text will be shown to worker to help him to solve the captcha correctly.
    /// For example: type red symbols only.
    pub text_captcha: String,
    pub pingback: Option<String>,
}

impl CaptchaArguments for TextCaptcha {
    fn to_request_params(&self, api_key: String) -> Result<Form, Error> {
        let mut request_body = Form::new()
            .text("key", api_key)
            .text("json", "1")
            .text("header_acao", "1")
            .text("soft_id", TWO_CAPTCHA_DEVELOPER_ID)
            .text("textcaptcha", self.text_captcha.clone());

        if let Some(lang) = &self.lang {
            request_body = request_body.text("lang", lang.clone());
        }

        if let Some(pingback) = &self.pingback {
            request_body = request_body.text("pingback", pingback.clone());
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

        Ok(request_body)
    }
}
