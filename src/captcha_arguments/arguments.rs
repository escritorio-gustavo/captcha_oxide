use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::error::Error;

pub trait CaptchaArguments<'a>: Clone + Serialize + Deserialize<'a> + Default + PartialEq {
    fn to_request_params(&self, api_key: String) -> Result<Form, Error>;

    fn get_initial_timeout_secs(&self) -> u64 {
        5
    }
}
