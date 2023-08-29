mod builder;

use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{
    arguments::{
        type_state::{page_url::PageUrlNotProvided, site_key::SiteKeyNotProvided},
        CaptchaArguments,
    },
    prelude::*,
    TWO_CAPTCHA_DEVELOPER_ID,
};

pub use builder::RecaptchaV3Builder;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
/// Represents the data needed to solve a reCaptcha V3 puzzle
///
/// # Example
/// ```
/// # use std::env;
/// use captcha_oxide::{
///     arguments::RecaptchaV3,
///     CaptchaSolver, RequestContent,
/// };
///
/// # #[tokio::main]
/// # pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # dotenv::dotenv();
/// let solver = CaptchaSolver::new("YOUR_API_KEY");
/// # let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());
///
/// let args = RecaptchaV3::builder()
///     .site_key("SOME_SITE_KEY")
/// #   .site_key("6Ld2sf4SAAAAAKSgzs0Q13IZhY02Pyo31S2jgOB5")
///     .page_url("SOME_URL")
/// #   .page_url("https://patrickhlauke.github.io/recaptcha/")
///     .min_score(0.3)
///     .build();
///
/// let solution = solver.solve(args).await?.solution;
/// let RequestContent::String(solution) = solution else {
///     unreachable!()
/// };
///
/// assert_ne!(solution, "");
/// # Ok(())
/// # }
/// ```
pub struct RecaptchaV3 {
    /// Full URL of the page where you see the captcha
    page_url: String,

    /// Value of the sitekey parameter you found on the page
    site_key: String,

    /// Whether or not the page uses Enterprise reCAPTCHA
    enterprise: Option<bool>,

    /// Domain used to load the captcha, e.g.: google.com or recaptcha.net
    domain: Option<String>,

    /// Value of the action parameter you found on the page
    action: Option<String>,

    /// The score needed for resolution
    min_score: Option<f32>,

    /// Callback URL where you wish to receive the response
    pingback: Option<String>,
}

impl RecaptchaV3 {
    pub fn builder() -> RecaptchaV3Builder<PageUrlNotProvided, SiteKeyNotProvided> {
        RecaptchaV3Builder::new()
    }
}

impl CaptchaArguments<'_> for RecaptchaV3 {
    fn to_request_params(&self, api_key: String) -> Result<Form> {
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

    fn get_initial_timeout(&self) -> Duration {
        Duration::from_secs(15)
    }
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::env;

    use crate::{arguments::RecaptchaV3, CaptchaSolver, RequestContent};

    #[tokio::test]
    #[ignore = "These tests should run all at once, as this will likely cause a 429 block from the 2captcha API"]
    async fn recaptcha_v3() {
        dotenv().unwrap();
        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let args = RecaptchaV3::builder()
            .site_key("6LcFcoAUAAAAAN7Um8IRZOtbzgsV5ei2meTmRi6m")
            .page_url("https://contactform7.com/contact/")
            .min_score(0.3)
            .build();

        let solution = solver.solve(args).await;

        assert!(solution.is_ok());

        let solution = solution.unwrap().solution;
        let RequestContent::String(solution) = solution else {
            unreachable!()
        };

        assert_ne!(solution, "");
    }
}
