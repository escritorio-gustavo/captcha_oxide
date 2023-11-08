mod empty_data;

pub mod arkose_labs_captcha;
pub mod geetest;
pub mod h_captcha;
pub mod normal_captcha;
pub mod recaptcha;
pub mod turnstile_captcha;

pub use catptcha_oxide_derive::Solution;

pub trait CaptchaTask: serde::Serialize {
    type Solution: for<'de> serde::Deserialize<'de> + Solution;
    type Builder: Default;

    /// Allows for building the request data for the 2captcha API
    /// while checking at compile time if all required fields were provided
    fn builder() -> Self::Builder {
        Self::Builder::default()
    }

    /// The amount of time that should be waited after creating a task to check
    /// if it is ready
    fn get_timeout(&self) -> std::time::Duration;
}

pub trait Solution {
    fn get_task_id(&self) -> u64;
    fn set_task_id(&mut self, task_id: u64);
}
