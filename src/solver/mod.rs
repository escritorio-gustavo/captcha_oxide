mod builder;
mod requests;

pub mod error;
pub mod language_pool;

pub use builder::SolverBuilder;
pub(crate) use error::SolveError;

use lazy_static::lazy_static;
use reqwest::Client;
use url::Url;

use crate::{captcha_types::CaptchaTask, prelude::*, solution::CaptchaSolution, SOFT_ID};

use self::{
    builder::NoApiKeyProvided,
    language_pool::LanguagePool,
    requests::create_task::{CreateTaskRequest, CreateTaskResponse},
    requests::get_balance::{GetBalanceRequest, GetBalanceResponse},
    requests::get_task_result::{GetTaskResultError, GetTaskResultRequest, GetTaskResultResponse},
};

lazy_static! {
    static ref API_URL: Url = Url::parse("https://api.2captcha.com").unwrap();
    static ref CLIENT: Client = Client::new();
}

#[derive(Default, Debug)]
pub struct CaptchaSolver {
    api_key: Box<str>,
    language_pool: LanguagePool,
    callback_url: Option<Url>,
}

impl CaptchaSolver {
    /// Returns a [`CaptchaSolver`] instance with the given api key
    pub fn new(api_key: impl Into<Box<str>>) -> Self {
        Self {
            api_key: api_key.into(),
            ..Default::default()
        }
    }

    /// Returns an instance of the [`SolverBuilder`], which ensures a [`Solver`]
    /// is built with an API key and provides methods to change its settings
    pub const fn builder() -> SolverBuilder<NoApiKeyProvided> {
        SolverBuilder::<NoApiKeyProvided>::new()
    }

    /// Sends a request to the 2captcha api to solve the given puzzle
    ///
    /// # Errors
    /// This method will error if the network request fails or if 2captcha returns
    /// an error.
    ///
    /// # Option
    /// This method will only ever return [`Ok(None)`] if you provide a [`CaptchaSolver::callback_url`]
    /// to the [`CaptchaSolver`] struct, otherwise a successful request will always return
    /// [`Ok(Some(CaptchaSolution))`]
    pub async fn solve<'a, T>(&self, task: T) -> Result<Option<CaptchaSolution<'a, T::Solution>>>
    where
        T: CaptchaTask,
    {
        let create_task = CreateTaskRequest {
            client_key: &self.api_key,
            task: &task,
            soft_id: SOFT_ID,
            callback_url: self.callback_url.as_ref(),
            language_pool: self.language_pool,
        };

        let task_id = Into::<std::result::Result<_, _>>::into(
            CLIENT
                .post(API_URL.join("/createTask")?)
                .header("Content-Type", "application/json")
                .json(&create_task)
                .send()
                .await?
                .json::<CreateTaskResponse>()
                .await?,
        )?;

        if self.callback_url.is_some() {
            return Ok(None);
        }

        tokio::time::sleep(task.get_timeout()).await;

        let task_result_url = API_URL.join("/getTaskResult")?;
        let task_result_request = GetTaskResultRequest {
            client_key: &self.api_key,
            task_id,
        };

        loop {
            let json = CLIENT
                .post(task_result_url.as_str())
                .header("Content-Type", "application/json")
                .json(&task_result_request)
                .send()
                .await?
                .text()
                .await?;

            if json.contains("errorCode") {
                let error = serde_json::from_str::<GetTaskResultError>(&json)?;
                let error: SolveError = error.error_code.as_ref().into();
                return Err(error.into());
            }

            let task_result: GetTaskResultResponse<'_, _> = serde_json::from_str(&json)?;

            if let GetTaskResultResponse::Ready(mut solution) = task_result {
                solution.task_id = task_id;
                return Ok(Some(solution));
            }

            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }

    /// Allows you to report to 2captcha on wether or not the solution was valid
    pub async fn report<'a, T>(
        &self,
        solution: CaptchaSolution<'a, T>,
        status: SolutionStatus,
    ) -> Result<()>
    where
        T: CaptchaTask,
    {
        let json = GetTaskResultRequest {
            client_key: &self.api_key,
            task_id: solution.task_id,
        };

        CLIENT
            .post(API_URL.join(status.report_endpoint())?)
            .json(&json)
            .send()
            .await?;

        Ok(())
    }

    /// Returns your account balance
    pub async fn get_balance(&self) -> Result<f32> {
        let request = GetBalanceRequest {
            client_key: self.api_key.as_ref(),
        };

        let balance = Into::<std::result::Result<_, _>>::into(
            CLIENT
                .post(API_URL.join("/getBalance")?)
                .json(&request)
                .send()
                .await?
                .json::<GetBalanceResponse>()
                .await?,
        )?;

        Ok(balance)
    }
}

pub enum SolutionStatus {
    Good,
    Bad,
}

impl SolutionStatus {
    pub const fn report_endpoint(&self) -> &str {
        match self {
            SolutionStatus::Good => "/reportCorrect",
            SolutionStatus::Bad => "/reportIncorrect",
        }
    }
}
