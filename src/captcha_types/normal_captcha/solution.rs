use std::borrow::Cow;

use crate::captcha_types::Solution;

#[derive(serde::Deserialize, Solution)]
pub struct NormalCaptchaSolution<'a> {
    #[serde(default = "Default::default")]
    task_id: u64,
    pub text: Cow<'a, str>,
}
