use std::borrow::Cow;

use crate::CaptchaTask;

#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 5, solution = super::solution::NormalCaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase", tag = "type", rename = "ImageToTextTask")]
pub struct NormalCaptcha<'a> {
    pub(super) body: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) phrase: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) case: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) numeric: Option<AnswerType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) math: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) min_length: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) max_length: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) comment: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
