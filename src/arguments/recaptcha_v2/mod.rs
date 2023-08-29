mod builder;
mod cookie;

use itertools::Itertools;
use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};

use crate::{
    arguments::{
        captcha_arguments::CaptchaArguments,
        proxy::Proxy,
        type_state::{page_url::PageUrlNotProvided, site_key::SiteKeyNotProvided},
    },
    prelude::*,
    TWO_CAPTCHA_DEVELOPER_ID,
};

pub use builder::RecaptchaV2Builder;
pub use cookie::Cookie;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
/// Represents the data needed to solve a reCaptcha V2 puzzle
///
/// # Example
/// ```
/// # use std::env;
/// use captcha_oxide::{
///     arguments::RecaptchaV2,
///     CaptchaSolver,
///     Solution,
/// };
///
/// # #[tokio::main]
/// # pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # dotenv::dotenv();
/// let solver = CaptchaSolver::new("YOUR_API_KEY");
/// # let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());
///
/// let args = RecaptchaV2::builder()
///     .site_key("SOME_SITE_KEY")
/// #   .site_key("6Ld2sf4SAAAAAKSgzs0Q13IZhY02Pyo31S2jgOB5")
///     .page_url("SOME_URL")
/// #   .page_url("https://patrickhlauke.github.io/recaptcha/")
///     .build();
///
/// let solution = solver.solve(args).await?.expect("Only None if pingback is set").solution;
/// let Solution::RecaptchaV2 { token, .. } = solution else {
///     unreachable!()
/// };
///
/// assert_ne!(token, "");
/// # Ok(())
/// # }
/// ```
pub struct RecaptchaV2 {
    /// Full URL of the page where you see the captcha
    page_url: String,

    /// Value of the sitekey parameter you found on the page
    site_key: String,

    /// Domain used to load the captcha, e.g.: google.com or recaptcha.net
    domain: Option<String>,

    /// Value of the data-s parameter you found on the page.
    /// Curenttly applicable for Google Search and other Google services.
    data_s: Option<String>,

    /// Your userAgent that will be used to solve the captcha
    user_agent: Option<String>,

    /// Callback URL where you wish to receive the response
    pingback: Option<String>,

    /// The URL to your proxy server
    proxy: Option<Proxy>,

    /// Whether or not the page uses Enterprise reCAPTCHA
    enterprise: Option<bool>,

    /// Whether or not the page uses Invisible reCAPTCHA
    invisible: Option<bool>,

    /// Cookies to be sent with your request
    cookies: Vec<Cookie>,
}

impl RecaptchaV2 {
    pub fn builder() -> RecaptchaV2Builder<PageUrlNotProvided, SiteKeyNotProvided> {
        RecaptchaV2Builder::new()
    }
}

impl CaptchaArguments<'_> for RecaptchaV2 {
    fn to_request_params(&self, api_key: String) -> Result<Form> {
        let mut request_body = Form::new()
            .text("key", api_key)
            .text("json", "1")
            .text("header_acao", "1")
            .text("soft_id", TWO_CAPTCHA_DEVELOPER_ID)
            .text("pageurl", self.page_url.clone())
            .text("googlekey", self.site_key.clone())
            .text("method", "userrecaptcha");

        if let Some(proxy) = &self.proxy {
            request_body = request_body
                .text("proxy", proxy.to_string())
                .text("proxytype", proxy.proxy_type.to_string());
        }

        if let Some(domain) = &self.domain {
            request_body = request_body.text("domain", domain.clone());
        }

        if let Some(data_s) = &self.data_s {
            request_body = request_body.text("data-s", data_s.clone());
        }

        if let Some(user_agent) = &self.user_agent {
            request_body = request_body.text("userAgent", user_agent.clone());
        }

        if let Some(pingback) = &self.pingback {
            request_body = request_body.text("pingback", pingback.clone());
        }

        if let Some(enterprise) = self.enterprise {
            request_body = request_body.text("enterprise", if enterprise { "1" } else { "0" });
        }

        if let Some(invisible) = self.invisible {
            request_body = request_body.text("invisible", if invisible { "1" } else { "0" });
        }

        if !self.cookies.is_empty() {
            let cookies = self
                .cookies
                .iter()
                .map(|x| format!("{}:{}", x.0, x.1))
                .join(";");

            request_body = request_body.text("cookies", cookies);
        }

        Ok(request_body)
    }

    fn get_initial_timeout(&self) -> Duration {
        Duration::from_secs(15)
    }

    crate::arguments::captcha_arguments::impl_methods!(RecaptchaV2);
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::env;

    use crate::{arguments::RecaptchaV2, CaptchaSolver, Solution};

    #[tokio::test]
    #[ignore = "These tests should run all at once, as this will likely cause a 429 block from the 2captcha API"]
    async fn recaptcha_v2() {
        dotenv().unwrap();
        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let args = RecaptchaV2::builder()
            .site_key("6Ld2sf4SAAAAAKSgzs0Q13IZhY02Pyo31S2jgOB5")
            .page_url("https://patrickhlauke.github.io/recaptcha/")
            .build();

        let solution = solver.solve(args).await;

        assert!(solution.is_ok());

        let solution = solution.unwrap().unwrap().solution;
        let Solution::RecaptchaV2 { token, .. } = solution else {
            unreachable!("Wrong enum variant")
        };

        assert_ne!(token, "");
    }
}
