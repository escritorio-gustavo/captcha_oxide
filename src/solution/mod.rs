use std::time::{Duration, Instant};

use itertools::Itertools;

use crate::{
    arguments::{captcha_type::CaptchaType, recaptcha_v2::Cookie},
    prelude::*,
    response::{CaptchaResponse, RequestContent},
    status::SolutionStatus,
    TWO_CAPTCHA_URL,
};

mod solution_enum;

pub use solution_enum::Solution;

/// This struct is returned by the [CaptchaSolver](crate::CaptchaSolver)'s `solve` method
/// and represents the solution to the captcha you submitted with
/// your [CaptchaArguments](crate::arguments::CaptchaArguments)
#[derive(Clone, Debug)]
pub struct CaptchaSolution {
    api_key: String,

    id: String,

    timestamp: Instant,

    /// The actual solution to the captcha challenge
    pub solution: Solution,
}

impl PartialEq for CaptchaSolution {
    fn eq(&self, other: &Self) -> bool {
        self.api_key == other.api_key && self.id == other.id && self.solution == other.solution
    }
}

impl CaptchaSolution {
    pub(crate) fn new(
        api_key: String,
        id: String,
        captcha_type: CaptchaType,
        response: CaptchaResponse,
    ) -> Self {
        let solution = match captcha_type {
            CaptchaType::HCaptcha => Solution::HCaptcha {
                token: response.request.request_as_string(),
                user_agent: response.user_agent,
            },
            CaptchaType::RecaptchaV2 => Solution::RecaptchaV2 {
                token: response.request.request_as_string(),
                cookies: response.cookies.map(|x| {
                    x.split(';')
                        .filter_map(|c| {
                            let (key, value) = c.split_once(':')?;
                            Some(Cookie(key.into(), value.into()))
                        })
                        .collect_vec()
                }),
            },
            _ => match response.request {
                RequestContent::String(token) => Solution::Token(token),
                RequestContent::GeetestResponse {
                    challenge,
                    validate,
                    seccode,
                } => Solution::Geetest {
                    challenge,
                    validate,
                    seccode,
                },
                RequestContent::GeetestV4Response {
                    captcha_id,
                    lot_number,
                    pass_token,
                    gen_time,
                    captcha_output,
                } => Solution::GeetestV4 {
                    captcha_id,
                    lot_number,
                    pass_token,
                    gen_time,
                    captcha_output,
                },
                RequestContent::CapyResponse {
                    captcha_key,
                    challenge_key,
                    answer,
                } => Solution::CapyCaptcha {
                    captcha_key,
                    challenge_key,
                    answer,
                },
            },
        };

        Self {
            api_key: api_key.to_owned(),
            id,
            solution,
            timestamp: Instant::now(),
        }
    }

    /// The amount of time elapsed since the solution was received.
    /// Useful if you know how long the captcha solution is valid for
    pub fn get_age(&self) -> Duration {
        self.timestamp.elapsed()
    }

    /// Use this method to report wether or not a captcha solution was valid.
    /// This helps increase the service's accuracy and refunds you for wrong
    /// solutions
    pub async fn report(&self, status: SolutionStatus) -> Result<()> {
        let action = match status {
            SolutionStatus::Good => "reportgood",
            SolutionStatus::Bad => "reportbad",
        };

        let url = reqwest::Url::parse(TWO_CAPTCHA_URL)?
            .join("res.php")?
            .as_str()
            .to_owned();
        let url = reqwest::Url::parse_with_params(
            &url,
            &[
                ("json", "1"),
                ("action", action),
                ("header_acao", "1"),
                ("id", &self.id),
                ("key", &self.api_key),
            ],
        )?;

        reqwest::get(url).await?.text().await?;
        Ok(())
    }
}
