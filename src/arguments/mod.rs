//! This module contains the parameters that must be sent
//! to 2captcha by the `CaptchaSolver::solve` method
//!
//! # Usage notes:
//!
//! * All implementers of the [`CaptchaArguments`] trait (and the trait
//!   itself) are re-exported by the top level of this module.
//! * All implementers of the [`CaptchaArguments`] trait have a `builder`
//!   method that gives you a builder using the typestate pattern to
//!   avoid inconsistent data
//! * Special parameter types like [`proxy::Proxy`] are not re-exported
//!   here

pub mod character_restrictions;
pub mod language;
pub mod proxy;
pub mod proxy_type;

pub mod captcha_arguments;
pub mod captcha_type;
pub mod capy_captcha;
pub mod geetest;
pub mod geetest_v4;
pub mod h_captcha;
pub mod key_captcha;
pub mod normal_captcha;
pub mod recaptcha_v2;
pub mod recaptcha_v3;
pub mod text_captcha;
pub mod type_state;

pub use captcha_arguments::CaptchaArguments;
pub use capy_captcha::CapyCaptcha;
pub use geetest::Geetest;
pub use geetest_v4::GeetestV4;
pub use h_captcha::HCaptcha;
pub use key_captcha::KeyCaptcha;
pub use normal_captcha::NormalCaptcha;
pub use recaptcha_v2::RecaptchaV2;
pub use recaptcha_v3::RecaptchaV3;
pub use text_captcha::TextCaptcha;
