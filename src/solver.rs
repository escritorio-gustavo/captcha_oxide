use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    captcha_arguments::arguments::CaptchaArguments,
    error::Error,
    response::{CaptchaResponse, RequestContent},
    solution::CaptchaSolution,
    TWO_CAPTCHA_URL,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CaptchaSolver {
    api_key: String,
}

impl CaptchaSolver {
    pub fn new(api_key: impl Into<String>) -> Self {
        CaptchaSolver {
            api_key: api_key.into(),
        }
    }

    pub async fn solve<T: CaptchaArguments>(&self, params: T) -> Result<CaptchaSolution, Error> {
        let client = Client::new();

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

        let task_id = response.request.request_as_string();
        tokio::time::sleep(Duration::from_secs(params.get_initial_timeout_secs())).await;

        let result_params: Vec<(&str, &str)> = vec![
            ("id", task_id.as_str()),
            ("key", &self.api_key),
            ("json", "1"),
            ("header_acao", "1"),
            ("action", "get"),
        ];

        let url = Url::parse(TWO_CAPTCHA_URL)?
            .join("res.php")?
            .as_str()
            .to_owned();
        let url = Url::parse_with_params(&url, &result_params)?;

        println!("{}", url.as_str());

        loop {
            let response = client
                .get(url.as_str())
                .send()
                .await
                .map_err(|e| e.without_url())?
                .text()
                .await
                .map_err(|e| e.without_url())?;
            let response: CaptchaResponse = serde_json::from_str(&response)?;

            match response {
                CaptchaResponse {
                    status: 1, request, ..
                } => {
                    return Ok(CaptchaSolution::new(
                        self.api_key.clone(),
                        task_id.clone(),
                        request,
                    ));
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
}
