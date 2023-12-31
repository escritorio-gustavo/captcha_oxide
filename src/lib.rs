//! A high level async library that allows you to use the 2captcha API
//! to solve various types of captcha puzzles
//!
//! # Example
//!
//! ```
//! use url::Url;
//! use captcha_oxide::{
//!     captcha_types::recaptcha::RecaptchaV3,
//!     CaptchaTask,
//!     CaptchaSolver,
//! };
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let solver = CaptchaSolver::new("YOUR_API_KEY");
//!
//! let args = RecaptchaV3::builder()
//!     .website_url("https://some.url/")
//!     .website_key("SOME_SITE_KEY")
//!     .min_score(0.3)
//!     .build()?;
//!
//! let solution = solver
//!     .solve(args)
//!     .await?
//!     .expect("Only `None` if pingback is set")
//!     .solution
//!     .g_recaptcha_response;
//!
//! assert!(!solution.is_empty());
//! # Ok(())
//! # }
//! ```

mod prelude;
pub(crate) const SOFT_ID: u16 = 4143;

pub mod captcha_types;
pub mod cookie;
pub mod error;
pub mod proxy;
pub mod solution;
pub mod solver;

pub use captcha_types::CaptchaTask;
pub use error::Error;
pub use solver::CaptchaSolver;
