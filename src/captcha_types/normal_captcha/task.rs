use std::borrow::Cow;

use crate::CaptchaTask;

use super::{
    builder::{NoBodyProvided, NormalCaptchaBuilder},
    solution::NormalCaptchaSolution,
};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalCaptcha<'a> {
    pub(super) body: Cow<'a, str>,
    pub(super) phrase: bool,
    pub(super) case: bool,
    pub(super) numeric: AnswerType,
    pub(super) math: bool,
    pub(super) min_length: u32,
    pub(super) max_length: u32,
    pub(super) comment: Option<Cow<'a, str>>,
    pub(super) img_instructions: Option<Cow<'a, str>>,
}

#[derive(serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum AnswerType {
    NoPreference = 0,
    Numeric = 1,
    Alphabetical = 2,
    AlphabeticalOrNumerical = 3,
    AlphaNumerical = 4,
}

impl<'a> CaptchaTask for NormalCaptcha<'a> {
    type Solution = NormalCaptchaSolution<'a>;
    type Builder = NormalCaptchaBuilder<'a, NoBodyProvided>;

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(10)
    }
}
