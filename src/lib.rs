//! A high level async library that allows you to use the 2captcha API
//! to solve various types of captcha puzzles
//!
//! # Example
//!
//! ```
//! use captcha_oxide::{
//!     Solution,
//!     arguments::{RecaptchaV3, CaptchaArguments},
//!     CaptchaSolver,
//! };
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let solver = CaptchaSolver::new("YOUR_API_KEY");
//!
//! let args = RecaptchaV3::builder()
//!     .site_key("SOME_SITE_KEY")
//!     .page_url("https://some.url/")
//!     .min_score(0.3)
//!     .build();
//!
//! let solution = solver
//!     .solve(args)
//!     .await?
//!     .expect("Only None if pingback is set")
//!     .solution;
//!
//! // If there isn't a variant named after your captcha type,
//! // it's because it only returns a token, so you should use
//! // the Token variant
//! let Solution::Token(solution) = solution else {
//!     unreachable!()
//! };
//!
//! assert_ne!(solution, "");
//! # Ok(())
//! # }
//! ```

pub use crate::arguments::CaptchaArguments;
pub use crate::error::Error;
pub use crate::solution::{CaptchaSolution, Solution};
pub use crate::solver::CaptchaSolver;

pub(crate) const TWO_CAPTCHA_URL: &str = "http://2captcha.com";
pub(crate) const TWO_CAPTCHA_DEVELOPER_ID: &str = "4143";
pub(crate) mod prelude;

pub mod arguments;
pub mod error;
pub mod response;
pub mod solution;
pub mod solver;
pub mod status;
