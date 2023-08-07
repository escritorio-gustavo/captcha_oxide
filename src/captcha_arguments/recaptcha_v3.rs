use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::{error::Error, TWO_CAPTCHA_DEVELOPER_ID};

use super::arguments::CaptchaArguments;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct RecaptchaV3 {
    /// Full URL of the page where you see the captcha
    ///
    /// This field is required
    pub page_url: String,

    /// Value of the sitekey parameter you found on the page
    ///
    /// This field is required
    pub site_key: String,

    /// Whether or not the page uses Enterprise reCAPTCHA
    pub enterprise: Option<bool>,

    /// Domain used to load the captcha, e.g.: google.com or recaptcha.net
    pub domain: Option<String>,

    /// Value of the action parameter you found on the page
    pub action: Option<String>,

    /// The score needed for resolution
    pub min_score: Option<f32>,

    /// Callback URL where you wish to receive the response
    pub pingback: Option<String>,
}

impl CaptchaArguments<'_> for RecaptchaV3 {
    fn to_request_params(&self, api_key: String) -> Result<Form, Error> {
        let mut request_body = Form::new()
            .text("key", api_key)
            .text("json", "1")
            .text("version", "v3")
            .text("header_acao", "1")
            .text("soft_id", TWO_CAPTCHA_DEVELOPER_ID)
            .text("pageurl", self.page_url.clone())
            .text("googlekey", self.site_key.clone())
            .text("method", "userrecaptcha");

        if let Some(enterprise) = self.enterprise {
            request_body = request_body.text("enterprise", if enterprise { "1" } else { "0" });
        }

        if let Some(domain) = &self.domain {
            request_body = request_body.text("domain", domain.clone());
        }

        if let Some(action) = &self.action {
            request_body = request_body.text("action", action.clone());
        }

        if let Some(min_score) = self.min_score {
            request_body = request_body.text("min_score", min_score.to_string());
        }

        if let Some(pingback) = &self.pingback {
            request_body = request_body.text("pingback", pingback.clone());
        }

        Ok(request_body)
    }

    fn get_initial_timeout_secs(&self) -> u64 {
        15
    }
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::env;

    use super::RecaptchaV3;
    use crate::{response::RequestContent, solver::CaptchaSolver};

    #[tokio::test]
    #[ignore = "These tests should run all at once, as this will likely cause a 429 block from the 2captcha API"]
    async fn recaptcha_v3() {
        dotenv().unwrap();
        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let args = RecaptchaV3 {
            site_key: "6LcFcoAUAAAAAN7Um8IRZOtbzgsV5ei2meTmRi6m".into(),
            page_url: "https://contactform7.com/contact/".into(),
            min_score: Some(0.3),
            ..Default::default()
        };

        let solution = solver.solve(args).await;

        assert!(solution.is_ok());

        let solution = solution.unwrap().solution;
        match solution {
            RequestContent::String(solution) => {
                assert_ne!(solution, "");
            }
            _ => unreachable!("Wrong enum variant"),
        }
    }
}
