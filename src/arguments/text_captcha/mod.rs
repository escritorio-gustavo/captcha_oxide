mod builder;
mod type_state;

use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::{
    arguments::{language::Language, CaptchaArguments},
    prelude::*,
    TWO_CAPTCHA_DEVELOPER_ID,
};

use builder::CaptchaTextNotProvided;
pub use builder::TextCaptchaBuilder;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
/// Represents the data needed to solve a text Captcha puzzle
///
/// # Example
/// ```
/// # use std::env;
/// use captcha_oxide::{
///     arguments::TextCaptcha,
///     CaptchaSolver,
///     RequestContent,
/// };
///
/// # #[tokio::main]
/// # pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # dotenv::dotenv();
/// let solver = CaptchaSolver::new("YOUR_API_KEY");
/// # let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());
///
/// let args = TextCaptcha::builder()
///     .text_captcha("What is 2 + 2?")
///     .build();
///
/// let solution = solver.solve(args).await?.solution;
///
/// let RequestContent::String(solution) = solution else {
///     unreachable!()
/// };
///
/// assert_eq!(solution, "4");
/// # Ok(())
/// # }
/// ```
pub struct TextCaptcha {
    /// Text that will be shown to the worker to help them solve the captcha correctly.
    /// For example: type red symbols only.
    text_captcha: String,

    /// Language code, such as `pt`, `en`, `es`
    language_code: Option<String>,

    /// The alphabet to be used
    language: Option<Language>,

    /// Callback URL where you wish to receive the response
    pingback: Option<String>,
}

impl TextCaptcha {
    pub fn builder() -> TextCaptchaBuilder<CaptchaTextNotProvided> {
        TextCaptchaBuilder::new()
    }
}

impl CaptchaArguments<'_> for TextCaptcha {
    fn to_request_params(&self, api_key: String) -> Result<Form> {
        let mut request_body = Form::new()
            .text("key", api_key)
            .text("json", "1")
            .text("header_acao", "1")
            .text("soft_id", TWO_CAPTCHA_DEVELOPER_ID)
            .text("textcaptcha", self.text_captcha.clone());

        if let Some(ref lang) = self.language_code {
            request_body = request_body.text("lang", lang.clone());
        }

        if let Some(ref pingback) = self.pingback {
            request_body = request_body.text("pingback", pingback.clone());
        }

        if let Some(ref language) = self.language {
            request_body = request_body.text(
                "language",
                match language {
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

    use crate::{arguments::TextCaptcha, CaptchaSolver, RequestContent};

    #[tokio::test]
    #[ignore = "These tests should run all at once, as this will likely cause a 429 block from the 2captcha API"]
    async fn text_captcha() {
        dotenv().unwrap();
        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let args = TextCaptcha::builder()
            .text_captcha("What is 2 + 2?")
            .build();

        let solution = solver.solve(args).await;

        assert!(solution.is_ok());

        let solution = solution.unwrap().solution;
        let RequestContent::String(solution) = solution else {
            unreachable!()
        };

        assert_eq!(solution, "4");
    }
}
