use std::borrow::Cow;

use crate::CaptchaTask;

#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 5)]
#[serde(rename_all = "camelCase", tag = "type", rename = "AudioTask")]
pub struct AudioCaptcha<'a> {
    pub(super) body: Cow<'a, str>,

    #[serde(rename = "lang")]
    pub(super) language: Language,
}

#[derive(Default, serde::Serialize, Debug)]
pub enum Language {
    #[serde(rename = "en")]
    #[default]
    English,

    #[serde(rename = "pt")]
    Portuguese,

    #[serde(rename = "fr")]
    French,

    #[serde(rename = "de")]
    German,

    #[serde(rename = "el")]
    Greek,

    #[serde(rename = "ru")]
    Russian,
}
