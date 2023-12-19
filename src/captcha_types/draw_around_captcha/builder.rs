use std::borrow::Cow;

use super::{type_state::*, DrawAroundCaptcha};

pub struct DrawAroundCaptchaBuilder<T, U, V> {
    body: T,
    comment: U,
    img_instructions: V,
}

impl<'a>
    DrawAroundCaptchaBuilder<BodyProvided<'a>, CommentProvided<'a>, ImgInstructionsProvided<'a>>
{
    pub fn build(self) -> DrawAroundCaptcha<'a> {
        DrawAroundCaptcha {
            body: self.body.0,
            comment: Some(self.comment.0),
            img_instructions: Some(self.img_instructions.0),
        }
    }
}

impl<'a> DrawAroundCaptchaBuilder<BodyProvided<'a>, CommentProvided<'a>, ImgInstructionsMissing> {
    pub fn build(self) -> DrawAroundCaptcha<'a> {
        DrawAroundCaptcha {
            body: self.body.0,
            comment: Some(self.comment.0),
            img_instructions: None,
        }
    }
}

impl<'a> DrawAroundCaptchaBuilder<BodyProvided<'a>, CommentMissing, ImgInstructionsProvided<'a>> {
    pub fn build(self) -> DrawAroundCaptcha<'a> {
        DrawAroundCaptcha {
            body: self.body.0,
            comment: None,
            img_instructions: Some(self.img_instructions.0),
        }
    }
}

impl DrawAroundCaptchaBuilder<BodyMissing, CommentMissing, ImgInstructionsMissing> {
    pub const fn new() -> Self {
        Self {
            body: BodyMissing,
            comment: CommentMissing,
            img_instructions: ImgInstructionsMissing,
        }
    }
}

impl Default for DrawAroundCaptchaBuilder<BodyMissing, CommentMissing, ImgInstructionsMissing> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, U, V> DrawAroundCaptchaBuilder<T, U, V> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    pub fn body(
        self,
        body: impl Into<Cow<'a, str>>,
    ) -> DrawAroundCaptchaBuilder<BodyProvided<'a>, U, V> {
        DrawAroundCaptchaBuilder {
            body: BodyProvided(body.into()),
            comment: self.comment,
            img_instructions: self.img_instructions,
        }
    }

    /// A comment will be shown to workers to help them solve the captcha properly.
    /// The [`DrawAroundCaptcha::comment`] property is required if
    /// [`DrawAroundCaptcha::img_instructions`] is missing.
    pub fn comment(
        self,
        comment: impl Into<Cow<'a, str>>,
    ) -> DrawAroundCaptchaBuilder<T, CommentProvided<'a>, V> {
        DrawAroundCaptchaBuilder {
            body: self.body,
            comment: CommentProvided(comment.into()),
            img_instructions: self.img_instructions,
        }
    }

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    /// The [`DrawAroundCaptcha::img_instructions`] property is required if
    /// the [`DrawAroundCaptcha::comment`] property is missing.
    pub fn img_instructions(
        self,
        img_instructions: impl Into<Cow<'a, str>>,
    ) -> DrawAroundCaptchaBuilder<T, U, ImgInstructionsProvided<'a>> {
        DrawAroundCaptchaBuilder {
            body: self.body,
            comment: self.comment,
            img_instructions: ImgInstructionsProvided(img_instructions.into()),
        }
    }
}
