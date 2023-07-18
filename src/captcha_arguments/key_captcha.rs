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
