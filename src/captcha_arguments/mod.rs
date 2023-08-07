//! This module contains the parameters that must be sent
//! to 2captcha by the `CaptchaSolver::solve` method
//!
//! # Usage notes:
//!
//! * All implementers of the [`arguments::CaptchaArguments`] trait
//! are re-exported by the top level of this module.
//! * All fields not typed as [`Option::<T>`] are required by the 2captcha
//! API and should not be filled by the [`Default`] trait
//! * Special parameter types like [`proxy_type::ProxyType`] are not re-exported
//! here

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

pub use capy_captcha::CapyCaptcha;
pub use geetest::Geetest;
pub use h_captcha::HCaptcha;
pub use key_captcha::KeyCaptcha;
pub use normal_captcha::NormalCaptcha;
pub use recaptcha_v2::RecaptchaV2;
pub use recaptcha_v3::RecaptchaV3;
pub use text_captcha::TextCaptcha;
