use std::borrow::Cow;

use super::{type_state::*, GridCaptcha};

pub struct GridCaptchaBuilder<T, U, V> {
    body: T,
    rows: Option<u8>,
    columns: Option<u8>,
    comment: U,
    img_instructions: V,
}

impl<'a> GridCaptchaBuilder<BodyProvided<'a>, CommentProvided<'a>, ImgInstructionsProvided<'a>> {
    pub fn build(self) -> GridCaptcha<'a> {
        GridCaptcha {
            body: self.body.0,
            rows: self.rows,
            columns: self.columns,
            comment: Some(self.comment.0),
            img_instructions: Some(self.img_instructions.0),
        }
    }
}

impl<'a> GridCaptchaBuilder<BodyProvided<'a>, CommentProvided<'a>, ImgInstructionsMissing> {
    pub fn build(self) -> GridCaptcha<'a> {
        GridCaptcha {
            body: self.body.0,
            rows: self.rows,
            columns: self.columns,
            comment: Some(self.comment.0),
            img_instructions: None,
        }
    }
}

impl<'a> GridCaptchaBuilder<BodyProvided<'a>, CommentMissing, ImgInstructionsProvided<'a>> {
    pub fn build(self) -> GridCaptcha<'a> {
        GridCaptcha {
            body: self.body.0,
            rows: self.rows,
            columns: self.columns,
            comment: None,
            img_instructions: Some(self.img_instructions.0),
        }
    }
}

impl GridCaptchaBuilder<BodyMissing, CommentMissing, ImgInstructionsMissing> {
    pub const fn new() -> Self {
        Self {
            body: BodyMissing,
            rows: None,
            columns: None,
            comment: CommentMissing,
            img_instructions: ImgInstructionsMissing,
        }
    }
}

impl Default for GridCaptchaBuilder<BodyMissing, CommentMissing, ImgInstructionsMissing> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, U, V> GridCaptchaBuilder<T, U, V> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    pub fn body(self, body: impl Into<Cow<'a, str>>) -> GridCaptchaBuilder<BodyProvided<'a>, U, V> {
        GridCaptchaBuilder {
            body: BodyProvided(body.into()),
            rows: self.rows,
            columns: self.columns,
            comment: self.comment,
            img_instructions: self.img_instructions,
        }
    }

    /// Number of grid rows
    pub fn rows(mut self, rows: Option<u8>) -> Self {
        self.rows = rows.map(Into::into);
        self
    }

    /// Number of grid columns
    pub fn columns(mut self, columns: Option<u8>) -> Self {
        self.columns = columns.map(Into::into);
        self
    }

    /// A comment will be shown to the workers to help them solve the captcha properly
    pub fn comment(
        self,
        comment: impl Into<Cow<'a, str>>,
    ) -> GridCaptchaBuilder<T, CommentProvided<'a>, V> {
        GridCaptchaBuilder {
            body: self.body,
            rows: self.rows,
            columns: self.columns,
            comment: CommentProvided(comment.into()),
            img_instructions: self.img_instructions,
        }
    }

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    pub fn img_instructions(
        self,
        img_instructions: impl Into<Cow<'a, str>>,
    ) -> GridCaptchaBuilder<T, U, ImgInstructionsProvided<'a>> {
        GridCaptchaBuilder {
            body: self.body,
            rows: self.rows,
            columns: self.columns,
            comment: self.comment,
            img_instructions: ImgInstructionsProvided(img_instructions.into()),
        }
    }
}
