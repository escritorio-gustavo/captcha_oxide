use std::borrow::Cow;

use super::{type_state::*, BoundingBoxCaptcha};

pub struct BoundingBoxCaptchaBuilder<T, U, V> {
    body: T,
    comment: U,
    img_instructions: V,
}

impl<'a>
    BoundingBoxCaptchaBuilder<BodyProvided<'a>, CommentProvided<'a>, ImgInstructionsProvided<'a>>
{
    pub fn build(self) -> BoundingBoxCaptcha<'a> {
        BoundingBoxCaptcha {
            body: self.body.0,
            comment: Some(self.comment.0),
            img_instructions: Some(self.img_instructions.0),
        }
    }
}

impl<'a> BoundingBoxCaptchaBuilder<BodyProvided<'a>, CommentProvided<'a>, ImgInstructionsMissing> {
    pub fn build(self) -> BoundingBoxCaptcha<'a> {
        BoundingBoxCaptcha {
            body: self.body.0,
            comment: Some(self.comment.0),
            img_instructions: None,
        }
    }
}

impl<'a> BoundingBoxCaptchaBuilder<BodyProvided<'a>, CommentMissing, ImgInstructionsProvided<'a>> {
    pub fn build(self) -> BoundingBoxCaptcha<'a> {
        BoundingBoxCaptcha {
            body: self.body.0,
            comment: None,
            img_instructions: Some(self.img_instructions.0),
        }
    }
}

impl BoundingBoxCaptchaBuilder<BodyMissing, CommentMissing, ImgInstructionsMissing> {
    pub const fn new() -> Self {
        Self {
            body: BodyMissing,
            comment: CommentMissing,
            img_instructions: ImgInstructionsMissing,
        }
    }
}

impl Default for BoundingBoxCaptchaBuilder<BodyMissing, CommentMissing, ImgInstructionsMissing> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, U, V> BoundingBoxCaptchaBuilder<T, U, V> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    pub fn body(
        self,
        body: impl Into<Cow<'a, str>>,
    ) -> BoundingBoxCaptchaBuilder<BodyProvided<'a>, U, V> {
        BoundingBoxCaptchaBuilder {
            body: BodyProvided(body.into()),
            comment: self.comment,
            img_instructions: self.img_instructions,
        }
    }

    /// A comment will be shown to workers to help them solve the captcha properly.
    /// The [`BoundingBoxCaptcha::comment`] property is required if
    /// [`BoundingBoxCaptcha::img_instructions`] is missing.
    pub fn comment(
        self,
        comment: impl Into<Cow<'a, str>>,
    ) -> BoundingBoxCaptchaBuilder<T, CommentProvided<'a>, V> {
        BoundingBoxCaptchaBuilder {
            body: self.body,
            comment: CommentProvided(comment.into()),
            img_instructions: self.img_instructions,
        }
    }

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    /// The [`BoundingBoxCaptcha::img_instructions`] property is required if
    /// the [`BoundingBoxCaptcha::comment`] property is missing.
    pub fn img_instructions(
        self,
        img_instructions: impl Into<Cow<'a, str>>,
    ) -> BoundingBoxCaptchaBuilder<T, U, ImgInstructionsProvided<'a>> {
        BoundingBoxCaptchaBuilder {
            body: self.body,
            comment: self.comment,
            img_instructions: ImgInstructionsProvided(img_instructions.into()),
        }
    }
}
