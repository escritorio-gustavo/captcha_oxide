use reqwest::multipart::Form;
use serde::Serialize;

use crate::error::Error;

pub trait CaptchaArguments: Clone + Serialize + Default {
    fn to_request_params(&self, api_key: String) -> Result<Form, Error>;

    fn get_initial_timeout_secs(&self) -> u64 {
        5
    }
}
