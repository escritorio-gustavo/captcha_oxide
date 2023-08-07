use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::{error::Error, TWO_CAPTCHA_DEVELOPER_ID};

use super::{arguments::CaptchaArguments, language::Language};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct TextCaptcha {
    /// Text will be shown to worker to help him to solve the captcha correctly.
    /// For example: type red symbols only.
    ///
    /// This field is required.
    pub text_captcha: String,

    /// Language code, such as `pt`, `en`, `es`
    pub lang: Option<String>,

    /// The alphabet to be used
    pub language: Option<Language>,

    /// Callback URL where you wish to receive the response
    pub pingback: Option<String>,
}

impl CaptchaArguments<'_> for TextCaptcha {
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

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::env;

    use super::TextCaptcha;
    use crate::{response::RequestContent, solver::CaptchaSolver};

    #[tokio::test]
    #[ignore = "These tests should run all at once, as this will likely cause a 429 block from the 2captcha API"]
    async fn text_captcha() {
        dotenv().unwrap();
        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let args = TextCaptcha {
            text_captcha: "What is 2 + 2?".into(),
            ..Default::default()
        };

        let solution = solver.solve(args).await;

        assert!(solution.is_ok());

        let solution = solution.unwrap().solution;
        match solution {
            RequestContent::String(solution) => {
                assert_eq!(solution, "4");
            }
            _ => unreachable!("Wrong enum variant"),
        }
    }
}
