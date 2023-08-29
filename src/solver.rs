use std::time::Duration;

use reqwest::Client;
use url::Url;

use crate::{
    arguments::captcha_arguments::CaptchaArguments,
    prelude::*,
    response::{CaptchaResponse, RequestContent},
    solution::CaptchaSolution,
    TWO_CAPTCHA_URL,
};

/// This struct is responsible for sending 2captcha the request for
/// solving your captcha puzzles.
///
/// You must instantiate it with your API key and use the `solve` method
/// to get the solution to your puzzle
///
/// # Example
/// ```
/// use std::env;
/// use captcha_oxide::{
///     CaptchaSolver,
///     arguments::RecaptchaV3,
///     Solution,
/// };
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # dotenv::dotenv();
/// let solver = CaptchaSolver::new("YOUR_API_KEY");
/// # let solver = CaptchaSolver::new(env::var("API_KEY")?);
///
/// let args = RecaptchaV3::builder()
///     .site_key("6LcFcoAUAAAAAN7Um8IRZOtbzgsV5ei2meTmRi6m")
///     .page_url("https://contactform7.com/contact/")
///     .min_score(0.3)
///     .build();
///
/// let solution = solver.solve(args).await?.expect("Only None if pingback is set").solution;
/// let Solution::Token(solution) = solution else {
///     unreachable!()
/// };
///
/// assert_ne!(solution, "");
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct CaptchaSolver {
    api_key: String,
}

impl CaptchaSolver {
    pub fn new(api_key: impl Into<String>) -> Self {
        CaptchaSolver {
            api_key: api_key.into(),
        }
    }

    /// Sends the data provided to the 2captcha API and returns the results
    ///
    /// # Option
    /// This method will only ever return `Ok(None)` if you provide a
    /// `pingback` when building the captcha data
    ///
    /// # Errors
    /// This method will error if
    /// * 2Captcha fails to create a puzzle task
    /// * 2Captcha fails to solve the puzzle
    /// * A network error happens (reqwest error)
    /// * The data returned by 2Captcha is not valid JSON
    /// # Example
    /// ```
    /// use std::env;
    /// use captcha_oxide::{
    ///     CaptchaSolver,
    ///     arguments::RecaptchaV3,
    ///     Solution,
    /// };
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # dotenv::dotenv();
    /// let solver = CaptchaSolver::new("YOUR_API_KEY");
    /// # let solver = CaptchaSolver::new(env::var("API_KEY")?);
    ///
    /// let args = RecaptchaV3::builder()
    ///     .site_key("6LcFcoAUAAAAAN7Um8IRZOtbzgsV5ei2meTmRi6m")
    ///     .page_url("https://contactform7.com/contact/")
    ///     .min_score(0.3)
    ///     .build();
    ///
    /// let solution = solver.solve(args).await?.expect("Only None if pingback is set").solution;
    /// let Solution::Token(solution) = solution else {
    ///     unreachable!()
    /// };
    ///
    /// assert_ne!(solution, "");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn solve<'a>(
        &self,
        params: impl CaptchaArguments<'a>,
    ) -> Result<Option<CaptchaSolution>> {
        let client = Client::new();

        let task = self.create_task(&params, &client).await?;
        let task_id = task.request.request_as_string();

        tokio::time::sleep(params.get_initial_timeout()).await;

        let url = self.create_result_url(&task_id)?;

        if params.is_pingback() {
            return Ok(None);
        }

        loop {
            let response = self.get_result(&client, &url).await?;

            match response {
                CaptchaResponse { status: 1, .. } => {
                    let solution = CaptchaSolution::new(
                        self.api_key.clone(),
                        task_id.clone(),
                        params.get_captcha_type(),
                        response,
                    );

                    return Ok(Some(solution));
                }
                // I am not checking if it equals "CAPTCHA_NOT_READY" because
                // there was (as of writing this comment) a typo in the API's
                // response, returning "CAPCHA_NOT_READY", which, if fixed, will
                // break this check
                CaptchaResponse {
                    request: RequestContent::String(request),
                    ..
                } if request.ends_with("NOT_READY") => {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
                CaptchaResponse {
                    request,
                    error_text,
                    ..
                } => {
                    return Err(Error::TwoCaptchaError {
                        error_code: request.request_as_string(),
                        error_description: error_text,
                    });
                }
            }
        }
    }

    fn create_result_url(&self, task_id: &str) -> Result<Url> {
        let result_params: Vec<(&str, &str)> = vec![
            ("id", task_id),
            ("key", &self.api_key),
            ("json", "1"),
            ("header_acao", "1"),
            ("action", "get"),
        ];
        let url = Url::parse(TWO_CAPTCHA_URL)?
            .join("res.php")?
            .as_str()
            .to_owned();

        Ok(Url::parse_with_params(&url, &result_params)?)
    }

    async fn get_result(&self, client: &Client, url: &Url) -> Result<CaptchaResponse> {
        let response = client
            .get(url.as_str())
            .send()
            .await
            .map_err(|e| e.without_url())?
            .text()
            .await
            .map_err(|e| e.without_url())?;

        let response: CaptchaResponse = serde_json::from_str(&response)?;
        Ok(response)
    }

    async fn create_task<'a>(
        &self,
        params: &impl CaptchaArguments<'a>,
        client: &Client,
    ) -> Result<CaptchaResponse> {
        let url = Url::parse(TWO_CAPTCHA_URL)?.join("in.php")?;
        let request_params = params.to_request_params(self.api_key.clone())?;
        let response = client
            .post(url)
            .multipart(request_params)
            .send()
            .await?
            .text()
            .await?;
        let response: CaptchaResponse = serde_json::from_str(&response)?;

        if response.status != 1 {
            return Err(Error::TwoCaptchaError {
                error_code: response.request.request_as_string(),
                error_description: response.error_text,
            });
        }

        Ok(response)
    }
}
