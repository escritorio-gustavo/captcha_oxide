//! This module contains the parameters that must be sent
//! to 2captcha by the `CaptchaSolver::solve` method
//!
//! # Usage notes:
//!
//! * All fields not typed as [`Option::<T>`] are required by the 2captcha
//! API and should not be filled by the [`Default`] trait
pub mod character_restrictions;
pub mod language;
pub mod proxy_type;

pub mod arguments;
pub mod capy_captcha;
pub mod geetest;
pub mod h_captcha;
pub mod key_captcha;
pub mod normal_captcha;
pub mod recaptcha_v2;
pub mod recaptcha_v3;
pub mod text_captcha;
