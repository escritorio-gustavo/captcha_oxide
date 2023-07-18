use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use super::{arguments::CaptchaArguments, proxy_type::ProxyType};
use crate::{error::Error, TWO_CAPTCHA_DEVELOPER_ID};

#[derive(Serialize, Deserialize, Clone, Default)]
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

impl CaptchaArguments for Geetest {
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
}
