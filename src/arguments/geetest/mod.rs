mod builder;
mod type_state;

use std::time::Duration;

use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use builder::{ChallengeNotProvided, GtNotProvided};

use crate::{
    arguments::{
        captcha_arguments::CaptchaArguments, proxy::Proxy, type_state::page_url::PageUrlNotProvided,
    },
    prelude::*,
    TWO_CAPTCHA_DEVELOPER_ID,
};

pub use builder::GeetestBuilder;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
/// Represents the data needed to solve a Geetest puzzle
///
/// # Example
/// ```
/// # use dotenv::dotenv;
/// use serde::{Deserialize, Serialize};
/// use std::{
///     env,
///     time::{SystemTime, UNIX_EPOCH},
/// };
/// use captcha_oxide::{
///     CaptchaSolver,
///     RequestContent,
///     arguments::Geetest
/// };
///
/// # #[derive(Serialize, Deserialize)]
/// # struct GeetestJson {
/// #     success: u8,
/// #     challenge: String,
/// #     gt: String,
/// #     new_captcha: bool,
/// # }
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # dotenv::dotenv();
/// # let timestamp = SystemTime::now()
/// #     .duration_since(UNIX_EPOCH)
/// #     .unwrap()
/// #     .as_millis();
/// #
///
/// let solver = CaptchaSolver::new("YOUR_API_KEY");
/// # let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());
/// # let url = format!("https://www.geetest.com/demo/gt/register-enFullpage-official?t={timestamp}");
/// # let json = reqwest::get(&url).await.unwrap().text().await.unwrap();
/// # let data: GeetestJson = serde_json::from_str(&json).unwrap();
///
/// let geetest_args = Geetest::builder()
///     .page_url("SOME URL")
/// #   .page_url(url)
///     .gt("DYNAMICALLY GENERATED")
/// #   .gt(data.gt)
///     .challenge("DYNAMICALLY GENERATED")
/// #   .challenge(data.challenge)
/// #   .new_captcha(data.new_captcha)
///     .build();
///
/// let solution = solver.solve(geetest_args).await?.solution;
/// let RequestContent::GeetestResponse { challenge, .. } = solution else {
///     unreachable!()
/// };
///
/// assert_ne!(challenge, "");
/// # Ok(())
/// # }
/// ```
pub struct Geetest {
    /// Public website key. You should be able to find it in the page's HTML
    gt: String,

    /// Full URL of the page where you see the captcha
    page_url: String,

    /// Challenge key. Warning, this field is dynamically generated, so you will
    /// need to get its value at runtime
    challenge: String,

    /// API domain
    api_server: Option<String>,

    /// In rare cases initGeetest can be called with offline parameter
    offline: Option<bool>,

    /// In rare cases initGeetest can be called with new_captcha parameter
    new_captcha: Option<bool>,

    /// Callback URL where you wish to receive the response
    pingback: Option<String>,

    /// The URL to your proxy server
    proxy: Option<Proxy>,

    /// Your userAgent that will be used to solve the captcha
    user_agent: Option<String>,
}

impl Geetest {
    pub fn builder() -> GeetestBuilder<GtNotProvided, PageUrlNotProvided, ChallengeNotProvided> {
        GeetestBuilder::new()
    }
}

impl CaptchaArguments<'_> for Geetest {
    fn to_request_params(&self, api_key: String) -> Result<Form> {
        let mut request_body = Form::new()
            .text("key", api_key)
            .text("json", "1")
            .text("header_acao", "1")
            .text("soft_id", TWO_CAPTCHA_DEVELOPER_ID)
            .text("challenge", self.challenge.clone())
            .text("pageurl", self.page_url.clone())
            .text("gt", self.gt.clone())
            .text("method", "geetest");

        if let Some(api_server) = &self.api_server {
            request_body = request_body.text("api_server", api_server.clone());
        }

        if let Some(offline) = &self.offline {
            request_body = request_body.text("offline", if *offline { "1" } else { "0" });
        }

        if let Some(new_captcha) = &self.new_captcha {
            request_body = request_body.text("new_captcha", if *new_captcha { "1" } else { "0" });
        }

        if let Some(pingback) = &self.pingback {
            request_body = request_body.text("pingback", pingback.clone());
        }

        if let Some(proxy) = &self.proxy {
            request_body = request_body
                .text("proxy", proxy.to_string())
                .text("proxytype", proxy.proxy_type.to_string());
        }

        if let Some(user_agent) = &self.user_agent {
            request_body = request_body.text("userAgent", user_agent.clone());
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
    use serde::{Deserialize, Serialize};
    use std::{
        env,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::Geetest;
    use crate::{CaptchaSolver, RequestContent};

    #[derive(Serialize, Deserialize)]
    struct GeetestJson {
        success: u8,
        challenge: String,
        gt: String,
        new_captcha: bool,
    }

    #[tokio::test]
    #[ignore = "These tests should run all at once, as this will likely cause a 429 block from the 2captcha API"]
    async fn geetest() {
        dotenv().unwrap();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        // Get dynamic parameters from the web page
        let url =
            format!("https://www.geetest.com/demo/gt/register-enFullpage-official?t={timestamp}");
        let json = reqwest::get(&url).await.unwrap().text().await.unwrap();
        let data: GeetestJson = serde_json::from_str(&json).unwrap();

        let args = Geetest::builder()
            .page_url(url)
            .gt(data.gt)
            .challenge(data.challenge)
            .new_captcha(data.new_captcha)
            .build();

        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let solution = solver.solve(args).await;

        assert!(solution.is_ok());

        let solution = solution.unwrap().solution;
        let RequestContent::GeetestResponse { challenge, .. } = solution else {
            unreachable!()
        };

        assert_ne!(challenge, "");
    }
}
