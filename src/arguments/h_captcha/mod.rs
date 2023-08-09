mod builder;
mod type_state;

use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use builder::{DataNotProvided, UserAgentNotProvided};

use crate::{
    arguments::{
        proxy::Proxy,
        type_state::{page_url::PageUrlNotProvided, site_key::SiteKeyNotProvided},
        CaptchaArguments,
    },
    prelude::*,
    TWO_CAPTCHA_DEVELOPER_ID,
};

pub use builder::HCaptchaBuilder;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
/// Represents the data needed to solve a hCaptcha puzzle
///
/// # Example
/// ```
/// use captcha_oxide::{
///     arguments::HCaptcha,
///     CaptchaSolver,
///     RequestContent
/// };
/// # use std::env;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let solver = CaptchaSolver::new("YOUR_API_KEY");
/// # dotenv::dotenv();
/// # let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());
///
/// let args = HCaptcha::builder()
///     .site_key("SITE_KEY")
/// #   .site_key("13257c82-e129-4f09-a733-2a7cb3102832")
///     .page_url("SOME_URL")
/// #   .page_url("https://dashboard.hcaptcha.com/signup")
///     .build();
///
/// let solution = solver.solve(args).await?.solution;
/// let RequestContent::String(solution) = solution else {
///     unreachable!();
/// };
///
/// assert_ne!(solution, "");
/// # Ok(())
/// # }
/// ```
pub struct HCaptcha {
    /// Value of the data-sitekey attribute found in the page's HTML
    site_key: String,

    /// Full URL of the page where you see the captcha
    page_url: String,

    /// Whether or not the captcha you are dealing with the invisible
    /// version of hCaptcha. This is pretty rare as of mid 2023
    invisible: Option<bool>,

    /// Domain used to load the captcha, e.g.: hcaptcha.com or js.hcaptcha.com
    domain: Option<String>,

    /// Custom data that is used in some implementations of hCaptcha,
    /// mostly with invisible hCaptcha. In most cases you see it as rqdata
    /// inside the page's network requests.
    ///
    /// Important: you MUST provide a `userAgent` if you submit a captcha
    /// with the data paramater. The value should match the User-Agent
    /// you use when interacting with the target website.
    data: Option<String>,

    /// Your userAgent that will be used to solve the captcha
    user_agent: Option<String>,

    /// Callback URL where you wish to receive the response
    pingback: Option<String>,

    /// The URL to your proxy server
    proxy: Option<Proxy>,
}

impl HCaptcha {
    pub fn builder() -> HCaptchaBuilder<
        PageUrlNotProvided,
        SiteKeyNotProvided,
        DataNotProvided,
        UserAgentNotProvided,
    > {
        HCaptchaBuilder::new()
    }
}

impl CaptchaArguments<'_> for HCaptcha {
    fn to_request_params(&self, api_key: String) -> Result<Form> {
        let mut request_body = Form::new()
            .text("key", api_key)
            .text("json", "1")
            .text("header_acao", "1")
            .text("soft_id", TWO_CAPTCHA_DEVELOPER_ID)
            .text("pageurl", self.page_url.clone())
            .text("sitekey", self.site_key.clone())
            .text("method", "hcaptcha");

        if let Some(invisible) = &self.invisible {
            request_body = request_body.text("invisible", if *invisible { "1" } else { "0" });
        }

        if let Some(domain) = &self.domain {
            request_body = request_body.text("domain", domain.clone());
        }

        if let Some(data) = &self.data {
            request_body = request_body.text("data", data.clone());
        }

        if let Some(user_agent) = &self.user_agent {
            request_body = request_body.text("userAgent", user_agent.clone());
        }

        if let Some(pingback) = &self.pingback {
            request_body = request_body.text("pingback", pingback.clone());
        }

        if let Some(proxy) = &self.proxy {
            request_body = request_body
                .text("proxy", proxy.to_string())
                .text("proxytype", proxy.proxy_type.to_string());
        }

        Ok(request_body)
    }
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::env;

    use super::HCaptcha;
    use crate::solver::CaptchaSolver;

    #[tokio::test]
    #[ignore = "These tests should run all at once, as this will likely cause a 429 block from the 2captcha API"]
    async fn h_captcha() {
        dotenv().unwrap();
        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let args = HCaptcha::builder()
            .site_key("13257c82-e129-4f09-a733-2a7cb3102832")
            .page_url("https://dashboard.hcaptcha.com/signup")
            .build();

        let solution = solver.solve(args).await;

        assert!(solution.is_ok());

        let solution = solution.unwrap().solution.request_as_string();
        assert_ne!(solution, "");
    }
}
