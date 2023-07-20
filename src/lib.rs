//! A high level async library that allows you to use the 2captcha API
//! to solve various types of captcha puzzles
//!
//! # Example
//!
//! ```no_run
//! use captcha_oxide::{
//!     solver::CaptchaSolver,
//!     captcha_arguments::recaptcha_v3::RecaptchaV3,
//!     response::RequestContent,
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     let solver = CaptchaSolver::new("YOUR_API_KEY");
//!     
//!     let args = RecaptchaV3 {
//!         site_key: "SOME_SITE_KEY".into(),
//!         page_url: "https://some.url/".into(),
//!         min_score: Some(0.3),
//!         ..Default::default()
//!     };
//!     
//!     match solver.solve(args).await {
//!         Ok(solution) => {
//!             // If there isn't a variant named after your captcha type,
//!             // it's because it only returns a token, so you should use
//!             // ths String variant
//!             match solution.solution {
//!                 RequestContent::String(plain_text_solution) => {
//!                     assert_ne!(plain_text_solution, "");
//!                 },
//!                 _ => unreachable!()
//!             }
//!         },
//!         Err(e) => {
//!             todo!("Handle your error");
//!         },
//!     };
//! }
//! ```
pub use crate::error::Error;
pub use crate::response::RequestContent;
pub use crate::solution::CaptchaSolution;
pub use crate::solver::CaptchaSolver;
pub use crate::status::SolutionStatus;

pub(crate) const TWO_CAPTCHA_URL: &str = "http://2captcha.com";
pub(crate) const TWO_CAPTCHA_DEVELOPER_ID: &str = "4143";

pub mod captcha_arguments;
pub mod error;
pub mod response;
pub mod solution;
pub mod solver;
pub mod status;
