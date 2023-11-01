mod prelude;
pub(crate) const SOFT_ID: u16 = 4143;

pub mod error;
pub mod proxy;
pub mod solver;
pub mod task;

pub use error::*;
pub use solver::Solver;
pub use task::CaptchaTask;
