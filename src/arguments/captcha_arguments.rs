use std::time::Duration;

use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

use super::captcha_type::CaptchaType;

/// Types used to send captcha data to 2captcha must implement this trait
/// The methods in this trait are for internal use in the [`CaptchaSolver`](crate::CaptchaSolver)'s
/// `solve` method. You shold not use them or expect changes to them to be
/// considered a breaking change
pub trait CaptchaArguments<'a>:
    std::fmt::Debug + Clone + Serialize + Deserialize<'a> + PartialEq
{
    fn get_captcha_type(&self) -> CaptchaType;

    /// Converts the data stored in the implementer into `multipart/form-data` to be sent
    /// to the 2captcha API through a POST request
    ///
    /// # Warning
    /// Methods in the [`CaptchaArguments`] trait are not meant to be used
    /// by users of the library, they are only for usage in
    /// the [`CaptchaSolver`](crate::CaptchaSolver)'s `solve` method.
    fn to_request_params(&self, api_key: String) -> Result<Form>;

    /// The amount of time to be waited before checking for a solution
    ///
    /// # Warning
    /// Methods in the [`CaptchaArguments`] trait are not meant to be used
    /// by users of the library, they are only for usage in
    /// the [`CaptchaSolver`](crate::CaptchaSolver)'s `solve` method.
    fn get_initial_timeout(&self) -> Duration {
        Duration::from_secs(5)
    }

    fn is_pingback(&self) -> bool;
}

#[macro_export]
macro_rules! impl_methods {
    ($x: ident) => {
        fn get_captcha_type(&self) -> $crate::arguments::captcha_type::CaptchaType {
            super::captcha_type::CaptchaType::$x
        }

        fn is_pingback(&self) -> bool {
            self.pingback.is_some()
        }
    };
}

pub(crate) use impl_methods;
