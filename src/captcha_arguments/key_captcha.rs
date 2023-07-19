use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::{error::Error, TWO_CAPTCHA_DEVELOPER_ID};

use super::arguments::CaptchaArguments;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct KeyCaptcha {
    pub user_id: String,
    pub session_id: String,
    pub server_sign: String,
    pub server_sign2: String,
    pub page_url: String,
    pub pingback: Option<String>,
}

impl CaptchaArguments for KeyCaptcha {
    fn to_request_params(&self, api_key: String) -> Result<Form, Error> {
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

    fn get_initial_timeout_secs(&self) -> u64 {
        15
    }
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use regex::Regex;
    use std::env;

    use super::KeyCaptcha;
    use crate::solver::CaptchaSolver;

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

        let args = KeyCaptcha {
            page_url: url.into(),
            user_id,
            session_id,
            server_sign,
            server_sign2,
            ..Default::default()
        };

        let solution = solver.solve(args).await;

        assert!(solution.is_ok());

        let solution = solution.unwrap().solution.request_as_string();
        assert_ne!(solution, "");
    }
}
