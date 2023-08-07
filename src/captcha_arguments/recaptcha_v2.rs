use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use super::{arguments::CaptchaArguments, proxy_type::ProxyType};
use crate::{error::Error, TWO_CAPTCHA_DEVELOPER_ID};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct RecaptchaV2 {
    /// Full URL of the page where you see the captcha
    ///
    /// This field is required
    pub page_url: String,

    /// Value of the sitekey parameter you found on the page
    ///
    /// This field is required
    pub site_key: String,

    /// Domain used to load the captcha, e.g.: google.com or recaptcha.net
    pub domain: Option<String>,

    /// Your cookies that will be passed to the worker who will solve the captha.
    /// This causes the worker's cookies to be returned in the response.
    /// Format: KEY:Value, separator: semicolon, example: `KEY1:Value1;KEY2:Value2;`
    ///
    /// # Warning
    /// Upon reading 2captcha docs again, I realized this changes the
    /// response format, which will break the JSON deserialization.
    /// Using this is highly discouraged as it has not been tested and will
    /// most likely fail.
    pub cookies: Option<String>,

    /// Value of the data-s parameter you found on the page.
    /// Curenttly applicable for Google Search and other Google services.
    pub data_s: Option<String>,

    /// Your userAgent that will be used to solve the captcha
    pub user_agent: Option<String>,

    /// Callback URL where you wish to receive the response
    pub pingback: Option<String>,

    /// The URL to your proxy server
    /// Format: login:password@ip_address:port
    pub proxy: Option<String>,

    /// The type of proxy
    pub proxy_type: Option<ProxyType>,

    /// Whether or not the page uses Enterprise reCAPTCHA
    pub enterprise: Option<bool>,

    /// Whether or not the page uses Invisible reCAPTCHA
    pub invisible: Option<bool>,
}

impl CaptchaArguments<'_> for RecaptchaV2 {
    fn to_request_params(&self, api_key: String) -> Result<Form, Error> {
        let mut request_body = Form::new()
            .text("key", api_key)
            .text("json", "1")
            .text("header_acao", "1")
            .text("soft_id", TWO_CAPTCHA_DEVELOPER_ID)
            .text("pageurl", self.page_url.clone())
            .text("googlekey", self.site_key.clone())
            .text("method", "userrecaptcha");

        if let Some(proxy) = &self.proxy {
            request_body = request_body.text("proxy", proxy.clone());
        }

        if let Some(domain) = &self.domain {
            request_body = request_body.text("domain", domain.clone());
        }

        if let Some(cookies) = &self.cookies {
            request_body = request_body.text("cookies", cookies.clone());
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

        if let Some(proxy_type) = &self.proxy_type {
            request_body = request_body.text("proxytype", proxy_type.to_string());
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

    use super::RecaptchaV2;
    use crate::{response::RequestContent, solver::CaptchaSolver};

    #[tokio::test]
    #[ignore = "These tests should run all at once, as this will likely cause a 429 block from the 2captcha API"]
    async fn recaptcha_v2() {
        dotenv().unwrap();
        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let args = RecaptchaV2 {
            site_key: "6Ld2sf4SAAAAAKSgzs0Q13IZhY02Pyo31S2jgOB5".into(),
            page_url: "https://patrickhlauke.github.io/recaptcha/".into(),
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
