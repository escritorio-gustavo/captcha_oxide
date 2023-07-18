use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use super::{arguments::CaptchaArguments, proxy_type::ProxyType};
use crate::{error::Error, TWO_CAPTCHA_DEVELOPER_ID};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RecaptchaV2 {
    pub page_url: String,
    pub site_key: String,
    pub proxy: Option<String>,
    pub domain: Option<String>,
    pub cookies: Option<String>,
    pub data_s: Option<String>,
    pub user_agent: Option<String>,
    pub pingback: Option<String>,
    pub proxy_type: Option<ProxyType>,
    pub enterprise: Option<bool>,
    pub invisible: Option<bool>,
}

impl CaptchaArguments for RecaptchaV2 {
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
