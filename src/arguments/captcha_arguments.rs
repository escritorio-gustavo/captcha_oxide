use std::time::Duration;

use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// Types used to send captcha data to 2captcha must implement this trait
pub trait CaptchaArguments<'a>: Clone + Serialize + Deserialize<'a> + PartialEq {
    /// Converts the data stored in the implementer into `multipart/form-data` to be sent
    /// to the 2captcha API through a POST request
    fn to_request_params(&self, api_key: String) -> Result<Form>;

    /// The amount of seconds to be waited before checking for a solution
    fn get_initial_timeout_secs(&self) -> Duration {
        Duration::from_secs(5)
    }
}
