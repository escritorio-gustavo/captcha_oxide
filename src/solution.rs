use std::time::{Duration, Instant};

use crate::{prelude::*, response::RequestContent, status::SolutionStatus, TWO_CAPTCHA_URL};

/// This struct is returned by the [CaptchaSolver](crate::CaptchaSolver)'s `solve` method
/// and represents the solution to the captcha you submitted with
/// your [CaptchaArguments](crate::arguments::CaptchaArguments)
#[derive(Clone, Debug, serde::Serialize)]
pub struct CaptchaSolution {
    #[serde(skip)]
    api_key: String,

    id: String,

    #[serde(skip)]
    timestamp: Instant,

    /// The actual solution to the captcha challenge
    pub solution: RequestContent,
}

impl PartialEq for CaptchaSolution {
    fn eq(&self, other: &Self) -> bool {
        self.api_key == other.api_key && self.id == other.id && self.solution == other.solution
    }
}

impl CaptchaSolution {
    pub(crate) fn new(api_key: String, id: String, solution: RequestContent) -> Self {
        Self {
            api_key,
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
