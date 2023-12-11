use std::borrow::Cow;

use crate::CaptchaTask;

use super::type_state::*;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase", tag = "type", rename = "GridTask")]
pub struct GridCaptcha<'a> {
    pub(super) body: Cow<'a, str>,
    pub(super) rows: Option<u8>,
    pub(super) columns: Option<u8>,
    pub(super) comment: Option<Cow<'a, str>>,
    pub(super) img_instructions: Option<Cow<'a, str>>,
}

impl<'a> CaptchaTask for GridCaptcha<'a> {
    type Solution = super::solution::GridCaptchaSolution;
    type Builder =
        super::builder::GridCaptchaBuilder<BodyMissing, CommentMissing, ImgInstructionsMissing>;

    fn get_timeout(&self) -> std::time::Duration {
        todo!()
    }
}
