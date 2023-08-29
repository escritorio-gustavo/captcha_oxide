//! Represents the JSON data returned by the 2captcha API

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct CaptchaResponse {
    pub(crate) status: u8,
    pub(crate) request: RequestContent,
    #[serde(rename = "useragent")]
    pub(crate) user_agent: Option<String>,
    pub(crate) error_text: Option<String>,
    pub(crate) cookies: Option<String>,
}

/// Represents the possible data contained by the `request` field
/// that 2captcha returns
///
/// It's usually a string (denoted by the [`String`](RequestContent::String) variant),
/// but some captcha types return objects instead. Those are denoted as
/// (CaptchaType)Response, e.g.: [`GeetestResponse`](RequestContent::GeetestResponse),
/// [`CapyResponse`](RequestContent::CapyResponse)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Hash)]
#[serde(untagged)]
pub(crate) enum RequestContent {
    /// Represents a captcha answer that is composed only of a token,
    /// such as reCAPTCHA and hCaptcha
    String(String),
    GeetestResponse {
        #[serde(rename = "geetest_challenge")]
        challenge: String,

        #[serde(rename = "geetest_validate")]
        validate: String,

        #[serde(rename = "geetest_seccode")]
        seccode: String,
    },
    GeetestV4Response {
        captcha_id: String,
        lot_number: String,
        pass_token: String,
        gen_time: String,
        captcha_output: String,
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
    /// Convinence method to be used only internally
    /// in cases where you are absolutely sure you are
    /// dealing with a [`String`](RequestContent::String)
    /// variant and don't want to add unecessary pattern
    /// matching
    ///
    /// # Panics
    /// If called on any variant other than [`String`](RequestContent::String)
    pub(crate) fn request_as_string(&self) -> String {
        if let RequestContent::String(data) = self {
            data.to_owned()
        } else {
            panic!()
        }
    }
}
