use crate::arguments::recaptcha_v2::Cookie;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
/// Represents the solution to your captcha puzzle. If there is not a variant
/// with the name of your captcha type, use the [`Token`](Solution::Token) variant
pub enum Solution {
    /// Represents a captcha answer that is composed only of a token,
    /// such as reCAPTCHA and hCaptcha
    Token(String),
    RecaptchaV2 {
        token: String,
        cookies: Option<Vec<Cookie>>,
    },
    HCaptcha {
        token: String,
        user_agent: Option<String>,
    },
    Geetest {
        challenge: String,

        validate: String,

        seccode: String,
    },
    GeetestV4 {
        captcha_id: String,
        lot_number: String,
        pass_token: String,
        gen_time: String,
        captcha_output: String,
    },
    CapyCaptcha {
        captcha_key: String,
        challenge_key: String,
        answer: String,
    },
}
