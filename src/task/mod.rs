pub mod recaptcha_v2;

pub trait CaptchaTask: serde::Serialize {
    type Solution: for<'de> serde::Deserialize<'de> + Solution;
    type Builder: Default;

    /// Allows for building the request data for the 2captcha API
    /// while checking at compile time if all required fields were provided
    fn builder() -> Self::Builder;

    /// The amount of time that should be waited after creating a task to check
    /// if it is ready
    fn get_timeout(&self) -> std::time::Duration;
}

pub trait Solution {
    fn get_task_id(&self) -> u64;
    fn set_task_id(&mut self, task_id: u64);
}
