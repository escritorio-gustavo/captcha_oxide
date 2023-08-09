mod builder;
mod capy_version;
mod type_state;

use std::time::Duration;

use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use builder::VersionNotProvided;

use crate::{
    arguments::{
        proxy::Proxy,
        type_state::{page_url::PageUrlNotProvided, site_key::SiteKeyNotProvided},
        CaptchaArguments,
    },
    prelude::*,
    TWO_CAPTCHA_DEVELOPER_ID,
};

pub use builder::CapyCaptchaBuilder;
pub use capy_version::CapyVersion;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
/// Represents the data needed to solve a Capy Captcha puzzle
///
/// # Example
/// ```
/// use captcha_oxide::{
///     arguments::{
///         CapyCaptcha,
///         capy_captcha::CapyVersion
///     },
///     CaptchaSolver,
///     RequestContent
/// };
///
/// # use std::env;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # dotenv::dotenv();
/// let solver = CaptchaSolver::new("YOUR_API_KEY");
/// # let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());
///
/// let capy_args = CapyCaptcha::builder()
///     .site_key("PAGE_SITE_KEY")
/// #   .site_key("PUZZLE_Cme4hZLjuZRMYC3uh14C52D3uNms5w")
///     .page_url("SOME_URL")
/// #   .page_url("https://www.capy.me/account/signin")
///     .version(CapyVersion::Puzzle)
///     .build();
///
/// let solution = solver.solve(capy_args).await?.solution;
/// let RequestContent::CapyResponse { answer, .. } = solution else {
///     unreachable!();
/// };
///
/// assert_ne!(answer, "");
/// # Ok(())
/// # }
/// ```
pub struct CapyCaptcha {
    /// The website's captcha key. You should be able to find this
    /// informatino in the site's HTML.
    site_key: String,

    /// Full URL of the page where you see the captcha
    page_url: String,

    /// The kind of puzzle to be solved
    version: CapyVersion,

    /// The domain of the script's source URL
    api_server: Option<String>,

    /// Callback URL where you wish to receive the response
    pingback: Option<String>,

    /// The URL to your proxy server
    proxy: Option<Proxy>,
}

impl CapyCaptcha {
    pub fn builder(
    ) -> CapyCaptchaBuilder<PageUrlNotProvided, SiteKeyNotProvided, VersionNotProvided> {
        CapyCaptchaBuilder::new()
    }
}

impl CaptchaArguments<'_> for CapyCaptcha {
    fn to_request_params(&self, api_key: String) -> Result<Form> {
        let mut request_body = Form::new()
            .text("method", "capy")
            .text("header_acao", "1")
            .text("json", "1")
            .text("key", api_key)
            .text("version", self.version.to_string())
            .text("soft_id", TWO_CAPTCHA_DEVELOPER_ID)
            .text("captchakey", self.site_key.clone())
            .text("pageurl", self.page_url.clone());

        if let Some(api_server) = &self.api_server {
            request_body = request_body.text("api_server", api_server.clone());
        }

        if let Some(proxy) = &self.proxy {
            request_body = request_body
                .text("proxy", proxy.to_string())
                .text("proxytype", proxy.proxy_type.to_string());
        }

        Ok(request_body)
    }

    fn get_initial_timeout_secs(&self) -> Duration {
        Duration::from_secs(15)
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

        let args = CapyCaptcha::builder()
            .site_key("PUZZLE_Cme4hZLjuZRMYC3uh14C52D3uNms5w")
            .page_url("https://www.capy.me/account/signin")
            .version(CapyVersion::Puzzle)
            .build();

        let solution = solver.solve(args).await;

        assert!(solution.is_ok());

        let solution = solution.unwrap().solution;
        let RequestContent::CapyResponse { answer, .. } = solution else {
            unreachable!()
        };

        assert_ne!(answer, "");
    }
}
