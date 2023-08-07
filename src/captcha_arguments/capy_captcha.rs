use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use super::{arguments::CaptchaArguments, proxy_type::ProxyType};
use crate::{error::Error, TWO_CAPTCHA_DEVELOPER_ID};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
/// Represents the data needed to solve a Capy Captcha puzzle
///
/// # Example
/// ```
/// CapyCaptcha {
///     captcha_key: "PUZZLE_Cme4hZLjuZRMYC3uh14C52D3uNms5w".into(),
///     page_url: "https://www.capy.me/account/signin".into(),
///     version: CapyVersion::Puzzle
///     ..Default::default()
/// };
/// ```
pub struct CapyCaptcha {
    /// The website's captcha key. You should be able to find this
    /// informatino in the site's HTML.
    ///
    /// This field is required
    pub captcha_key: String,

    /// Full URL of the page where you see the captcha
    ///
    /// This field is required
    pub page_url: String,

    /// The kind of puzzle to be solved
    ///
    /// This field is required
    pub version: CapyVersion,

    /// The domain of the script's source URL
    pub api_server: Option<String>,

    /// Callback URL where you wish to receive the response
    pub pingback: Option<String>,

    /// The URL to your proxy server
    /// Format: login:password@ip_address:port
    pub proxy: Option<String>,

    /// The type of proxy
    pub proxy_type: Option<ProxyType>,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum CapyVersion {
    #[default]
    Puzzle,
    Avatar,
}

impl ToString for CapyVersion {
    fn to_string(&self) -> String {
        match self {
            CapyVersion::Puzzle => "puzzle",
            CapyVersion::Avatar => "avatar",
        }
        .into()
    }
}

impl CaptchaArguments<'_> for CapyCaptcha {
    fn to_request_params(&self, api_key: String) -> Result<Form, Error> {
        let mut request_body = Form::new()
            .text("method", "capy")
            .text("header_acao", "1")
            .text("json", "1")
            .text("key", api_key)
            .text("version", self.version.to_string())
            .text("soft_id", TWO_CAPTCHA_DEVELOPER_ID)
            .text("captchakey", self.captcha_key.clone())
            .text("pageurl", self.page_url.clone());

        if let Some(api_server) = &self.api_server {
            request_body = request_body.text("api_server", api_server.clone());
        }

        if let Some(proxy) = &self.proxy {
            request_body = request_body.text("proxy", proxy.clone());
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

    use super::*;
    use crate::{response::RequestContent, solver::CaptchaSolver};

    #[tokio::test]
    #[ignore = "These tests should run all at once, as this will likely cause a 429 block from the 2captcha API"]
    async fn capy_captcha() {
        dotenv().unwrap();
        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let args = CapyCaptcha {
            captcha_key: "PUZZLE_Cme4hZLjuZRMYC3uh14C52D3uNms5w".into(),
            page_url: "https://www.capy.me/account/signin".into(),
            version: CapyVersion::Puzzle,
            ..Default::default()
        };

        let solution = solver.solve(args).await;

        assert!(solution.is_ok());

        let solution = solution.unwrap().solution;
        match solution {
            RequestContent::CapyResponse { answer, .. } => {
                assert_ne!(answer, "");
            }
            _ => unreachable!("Wrong enum variant"),
        }
    }
}
