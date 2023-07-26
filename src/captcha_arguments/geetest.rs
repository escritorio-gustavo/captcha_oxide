use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use super::{arguments::CaptchaArguments, proxy_type::ProxyType};
use crate::{error::Error, TWO_CAPTCHA_DEVELOPER_ID};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Geetest {
    pub gt: String,
    pub page_url: String,
    pub challenge: String,
    pub api_server: Option<String>,
    pub offline: Option<bool>,
    pub new_captcha: Option<bool>,
    pub pingback: Option<String>,
    pub proxy: Option<String>,
    pub proxy_type: Option<ProxyType>,
    pub user_agent: Option<String>,
}

impl CaptchaArguments<'_> for Geetest {
    fn to_request_params(&self, api_key: String) -> Result<Form, Error> {
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
            request_body = request_body.text("proxy", proxy.clone());
        }

        if let Some(proxy_type) = &self.proxy_type {
            request_body = request_body.text("proxytype", proxy_type.to_string());
        }

        if let Some(user_agent) = &self.user_agent {
            request_body = request_body.text("userAgent", user_agent.clone());
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
    use serde::{Deserialize, Serialize};
    use std::{
        env,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::Geetest;
    use crate::{response::RequestContent, solver::CaptchaSolver};

    #[derive(Serialize, Deserialize, Clone)]
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

        let args = Geetest {
            page_url: url,
            gt: data.gt,
            challenge: data.challenge,
            new_captcha: Some(data.new_captcha),
            ..Default::default()
        };

        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let solution = solver.solve(args).await;

        assert!(solution.is_ok());

        let solution = solution.unwrap().solution;
        match solution {
            RequestContent::GeetestResponse { challenge, .. } => {
                assert_ne!(challenge, "");
            }
            _ => unreachable!("Wrong enum variant"),
        }
    }
}
