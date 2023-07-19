use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct CaptchaResponse {
    pub(crate) status: u8,
    pub(crate) request: RequestContent,
    pub(crate) error_text: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum RequestContent {
    String(String),
    GeetestResponse {
        #[serde(rename = "geetest_challenge")]
        challenge: String,

        #[serde(rename = "geetest_validate")]
        validate: String,

        #[serde(rename = "geetest_seccode")]
        seccode: String,
    },
    CapyResponse {
        #[serde(rename = "captchakey")]
        captcha_key: String,
        #[serde(rename = "challengekey")]
        challenge_key: String,
        answer: String,
    },
}

impl RequestContent {
    pub(crate) fn request_as_string(&self) -> String {
        if let RequestContent::String(data) = self {
            data.to_owned()
        } else {
            panic!()
        }
    }
}
