mod builder;
mod type_state;

use std::{time::Duration};

use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::{prelude::*, TWO_CAPTCHA_DEVELOPER_ID};

use self::builder::{
    ServerSign2NotProvided, ServerSignNotProvided, SessionIdNotProvided, UserIdNotProvided,
};

use super::{captcha_arguments::CaptchaArguments, type_state::page_url::PageUrlNotProvided};

pub use builder::KeyCaptchaBuilder;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
/// Represents the data needed to solve a KeyCaptcha puzzle
///
/// # Example
/// ```
/// # use std::env;
/// # use regex::Regex;
/// use captcha_oxide::{
///     Solution,
///     CaptchaSolver,
///     arguments::KeyCaptcha
/// };
/// #
/// # #[tokio::main]
/// # pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # dotenv::dotenv();
/// # let url = "https://www.keycaptcha.com/contact-us/";
/// # let html = reqwest::get(url).await.unwrap().text().await.unwrap();
/// #
/// # let user_id = Regex::new("var s_s_c_user_id = '([^']*)'")
/// #   .unwrap()
/// #   .captures(&html)
/// #   .unwrap()
/// #   .get(1)
/// #   .unwrap()
/// #   .as_str()
/// #   .to_string();
/// #
/// # let session_id = Regex::new("var s_s_c_session_id = '([^']*)'")
/// #   .unwrap()
/// #   .captures(&html)
/// #   .unwrap()
/// #   .get(1)
/// #   .unwrap()
/// #   .as_str()
/// #   .to_string();
/// #
/// # let server_sign = Regex::new("var s_s_c_web_server_sign = '([^']*)'")
/// #   .unwrap()
/// #   .captures(&html)
/// #   .unwrap()
/// #   .get(1)
/// #   .unwrap()
/// #   .as_str()
/// #   .to_string();
/// #
/// # let server_sign2 = Regex::new("var s_s_c_web_server_sign2 = '([^']*)'")
/// #   .unwrap()
/// #   .captures(&html)
/// #   .unwrap()
/// #   .get(1)
/// #   .unwrap()
/// #   .as_str()
/// #   .to_string();
///
/// let solver = CaptchaSolver::new("YOUR_API_KEY");
/// # let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());
///
/// let args = KeyCaptcha::builder()
///     .page_url("SOME_URL")
/// #   .page_url(url)
///     .user_id("DINAMICALLY_GENERATED")
/// #   .user_id(user_id)
///     .session_id("DINAMICALLY_GENERATED")
/// #   .session_id(session_id)
///     .server_sign("DINAMICALLY_GENERATED")
/// #   .server_sign(server_sign)
///     .server_sign2("DINAMICALLY_GENERATED")
/// #   .server_sign2(server_sign2)
///     .build();
///
/// let solution = solver.solve(args).await?.expect("Only None if pingback is set").solution;
/// let Solution::Token(solution) = solution else {
///     unreachable!();
/// };
///
/// assert_ne!(solution, "");
/// # Ok(())
/// # }
/// ```
pub struct KeyCaptcha {
    /// Full URL of the page where you see the captcha
    page_url: String,

    /// Value of the s_s_c_user_id parameter you found on the page's source code
    user_id: String,

    /// Value of the s_s_c_session_id parameter you found on the page's source code
    session_id: String,

    /// Value of the s_s_c_web_server_sign parameter you found on the page's source code
    server_sign: String,

    /// Value of the s_s_c_web_server_sign2 parameter you found on the page's source code
    server_sign2: String,

    /// Callback URL where you wish to receive the response
    pingback: Option<String>,
}

impl KeyCaptcha {
    pub fn builder() -> KeyCaptchaBuilder<
        PageUrlNotProvided,
        UserIdNotProvided,
        SessionIdNotProvided,
        ServerSignNotProvided,
        ServerSign2NotProvided,
    > {
        KeyCaptchaBuilder::new()
    }
}

impl CaptchaArguments<'_> for KeyCaptcha {
    fn to_request_params(&self, api_key: String) -> Result<Form> {
        let mut request_body = Form::new()
            .text("key", api_key)
            .text("method", "keycaptcha")
            .text("json", "1")
            .text("header_acao", "1")
            .text("soft_id", TWO_CAPTCHA_DEVELOPER_ID)
            .text("s_s_c_user_id", self.user_id.clone())
            .text("s_s_c_session_id", self.session_id.clone())
            .text("s_s_c_web_server_sign", self.server_sign.clone())
            .text("s_s_c_web_server_sign2", self.server_sign2.clone())
            .text("pageurl", self.page_url.clone());

        if let Some(pingback) = &self.pingback {
            request_body = request_body.text("pingback", pingback.clone());
        }

        Ok(request_body)
    }

    fn get_initial_timeout(&self) -> Duration {
        Duration::from_secs(15)
    }

    crate::arguments::captcha_arguments::impl_methods!(KeyCaptcha);
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use regex::Regex;
    use std::env;

    use super::KeyCaptcha;
    use crate::{CaptchaSolver, Solution};

    #[tokio::test]
    #[ignore = "These tests should run all at once, as this will likely cause a 429 block from the 2captcha API"]
    async fn key_captcha() {
        dotenv().unwrap();

        // Get dynamic parameters from the web page
        let url = "https://www.keycaptcha.com/contact-us/";
        let html = reqwest::get(url).await.unwrap().text().await.unwrap();

        let user_id = Regex::new("var s_s_c_user_id = '([^']*)'")
            .unwrap()
            .captures(&html)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string();

        let session_id = Regex::new("var s_s_c_session_id = '([^']*)'")
            .unwrap()
            .captures(&html)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string();

        let server_sign = Regex::new("var s_s_c_web_server_sign = '([^']*)'")
            .unwrap()
            .captures(&html)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string();

        let server_sign2 = Regex::new("var s_s_c_web_server_sign2 = '([^']*)'")
            .unwrap()
            .captures(&html)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string();

        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let args = KeyCaptcha::builder()
            .page_url(url)
            .user_id(user_id)
            .session_id(session_id)
            .server_sign(server_sign)
            .server_sign2(server_sign2)
            .build();

        let solution = solver.solve(args).await;

        assert!(solution.is_ok());

        let solution = solution.unwrap().unwrap().solution;
        let Solution::Token(solution) = solution else {
            unreachable!();
        };
        assert_ne!(solution, "");
    }
}
