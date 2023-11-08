use std::borrow::Cow;

use crate::captcha_types::CaptchaSolution;

#[derive(serde::Deserialize, CaptchaSolution)]
#[serde(rename_all = "camelCase")]
pub struct GeeTestV4Solution<'a> {
    #[serde(default = "Default::default")]
    task_id: u64,

    pub captcha_id: Cow<'a, str>,
    pub lot_number: Cow<'a, str>,
    pub pass_token: Cow<'a, str>,
    pub gen_time: Cow<'a, str>,
    pub captcha_output: Cow<'a, str>,
}
