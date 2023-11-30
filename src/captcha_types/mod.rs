mod empty_data;

pub mod amazon_captcha;
pub mod arkose_labs_captcha;
pub mod capy_captcha;
pub mod cut_captcha;
pub mod cyber_siara_captcha;
pub mod data_dome_captcha;
pub mod geetest;
pub mod h_captcha;
pub mod key_captcha;
pub mod lemin_captcha;
pub mod mt_captcha;
pub mod normal_captcha;
pub mod recaptcha;
pub mod rotate_captcha;
pub mod text_captcha;
pub mod turnstile_captcha;

use std::fmt::Debug;

pub(crate) use captcha_oxide_derive::CaptchaTask;

pub trait CaptchaTask: serde::Serialize {
    type Solution: CaptchaSolution;
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

pub trait CaptchaSolution: for<'de> serde::Deserialize<'de> + Debug {
    fn get_task_id(&self) -> u64;
    fn set_task_id(&mut self, task_id: u64);
}
