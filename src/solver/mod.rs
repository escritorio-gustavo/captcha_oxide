use lazy_static::lazy_static;

lazy_static! {
    static ref API_URL: Url = Url::parse("https://api.2captcha.com").unwrap();
    static ref CLIENT: Client = Client::new();
}

use reqwest::Client;
use url::Url;

use crate::{
    captcha_types::{CaptchaSolution, CaptchaTask},
    prelude::*,
    SOFT_ID,
};

use self::{
    builder::NoApiKeyProvided,
    create_task::{CreateTaskRequest, CreateTaskResponse},
    language_pool::LanguagePool,
    task_request::TaskRequest,
    task_result::{TaskResult, TaskResultResponse},
};

mod builder;
mod error;
mod task_request;

pub mod create_task;
pub mod language_pool;
pub mod task_result;

pub use builder::SolverBuilder;
pub(crate) use error::SolveError;

#[derive(Default, Debug)]
pub struct CaptchaSolver {
    api_key: Box<str>,
    language_pool: LanguagePool,
    callback_url: Option<Url>,
}

impl CaptchaSolver {
    /// Returns a [`Solver`] instance with the given api key
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
    /// This method will only ever return [`Ok(None)`] if you provide a `callback_url`
    /// to the [`Solver`] struct, otherwise a successful request will always return
    /// [`Ok(Some(CaptchaSolution))`]
    pub async fn solve<T>(&self, task: T) -> Result<Option<T::Solution>>
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
        let task_result_request = TaskRequest {
            client_key: &self.api_key,
            task_id,
        };

        loop {
            let task_result = Into::<std::result::Result<_, _>>::into(
                CLIENT
                    .post(task_result_url.as_str())
                    .header("Content-Type", "application/json")
                    .json(&task_result_request)
                    .send()
                    .await?
                    .json::<TaskResultResponse<T::Solution>>()
                    .await?,
            )?;

            if let TaskResult::Ready { mut solution, .. } = task_result {
                solution.set_task_id(task_id);
                return Ok(Some(solution));
            }

            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }

    /// Allows you to report to 2captcha on wether or not the solution was valid
    pub async fn report(
        &self,
        solution: &impl CaptchaSolution,
        status: SolutionStatus,
    ) -> Result<()> {
        let task_id = solution.get_task_id();
        let json = TaskRequest {
            client_key: &self.api_key,
            task_id,
        };

        CLIENT
            .post(API_URL.join(status.report_endpoint())?)
            .json(&json)
            .send()
            .await?;

        Ok(())
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
