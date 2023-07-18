use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use super::{arguments::CaptchaArguments, proxy_type::ProxyType};
use crate::{error::Error, TWO_CAPTCHA_DEVELOPER_ID};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CapyCaptcha {
    pub captcha_key: String,
    pub api_server: Option<String>,
    pub version: CapyVersion,
    pub page_url: String,
    pub pingback: Option<String>,
    pub proxy: Option<String>,
    pub proxy_type: Option<ProxyType>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
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

impl CaptchaArguments for CapyCaptcha {
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
}
