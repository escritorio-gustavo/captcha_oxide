mod builder;

use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::{arguments::proxy::Proxy, prelude::Result, CaptchaArguments, TWO_CAPTCHA_DEVELOPER_ID};

pub use builder::GeetestV4Builder;

use super::type_state::{page_url::PageUrlNotProvided, site_key::SiteKeyNotProvided};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
/// Represents the data needed to solve a Geetest V4 puzzle
///
/// # Example
/// ```
/// # use std::env;
/// use captcha_oxide::{
///     CaptchaSolver,
///     Solution,
///     arguments::GeetestV4
/// };
/// # #[tokio::main]
/// # pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # dotenv::dotenv();
/// let solver = CaptchaSolver::new("YOUR_API_KEY");
/// # let solver = CaptchaSolver::new(env::var("API_KEY")?);
///
/// let args = GeetestV4::builder()
///     .site_key("SITE_KEY")
/// #   .site_key("55c86e822ef5984cc0b03a3bbfd1a7c7")
///     .page_url("SOME_URL")
/// #   .page_url("https://auth.geetest.com/login/")
///     .build();
///
/// let solution = solver.solve(args).await?.expect("Only None if pingback is set").solution;
/// let Solution::GeetestV4 { captcha_output, .. } = solution else {
///     unreachable!()
/// };
///
/// assert_ne!(captcha_output, "");
/// # Ok(())
/// # }
/// ```
pub struct GeetestV4 {
    /// Value of the captcha_id parameter found in the page's HTML
    site_key: String,

    /// Full URL of the page where you see the captcha
    page_url: String,

    /// Callback URL where you wish to receive the response
    pingback: Option<String>,

    /// The URL to your proxy server
    proxy: Option<Proxy>,
}

impl GeetestV4 {
    pub fn builder() -> GeetestV4Builder<SiteKeyNotProvided, PageUrlNotProvided> {
        GeetestV4Builder::new()
    }
}

impl CaptchaArguments<'_> for GeetestV4 {
    fn to_request_params(&self, api_key: String) -> Result<Form> {
        let mut request_body = Form::new()
            .text("key", api_key)
            .text("json", "1")
            .text("header_acao", "1")
            .text("soft_id", TWO_CAPTCHA_DEVELOPER_ID)
            .text("pageurl", self.page_url.clone())
            .text("captcha_id", self.site_key.clone())
            .text("method", "geetest_v4");

        if let Some(ref pingback) = self.pingback {
            request_body = request_body.text("pingback", pingback.clone());
        }

        if let Some(ref proxy) = self.proxy {
            request_body = request_body
                .text("proxy", proxy.to_string())
                .text("proxytype", proxy.proxy_type.to_string());
        }

        Ok(request_body)
    }

    fn get_initial_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(15)
    }

    crate::arguments::captcha_arguments::impl_methods!(GeetestV4);
}

#[cfg(test)]
mod test {
    use crate::{arguments::GeetestV4, CaptchaSolver, Solution};
    use std::env;

    #[tokio::test]
    #[ignore = "These tests should run all at once, as this will likely cause a 429 block from the 2captcha API"]
    async fn geetest_v4() {
        dotenv::dotenv().unwrap();
        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let args = GeetestV4::builder()
            .site_key("55c86e822ef5984cc0b03a3bbfd1a7c7")
            .page_url("https://auth.geetest.com/login/")
            .build();

        let solution = solver.solve(args).await.unwrap().unwrap().solution;
        let Solution::GeetestV4 { captcha_output, .. } = solution else {
            unreachable!()
        };

        assert_ne!(captcha_output, "");
    }
}
