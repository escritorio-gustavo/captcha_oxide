use std::borrow::Cow;

use super::{AnswerType, NormalCaptcha};

pub struct NoBodyProvided;
pub struct Body<'a>(Cow<'a, str>);

pub struct NormalCaptchaBuilder<'a, T> {
    body: T,
    phrase: bool,
    case: bool,
    numeric: AnswerType,
    math: bool,
    min_length: u32,
    max_length: u32,
    comment: Option<Cow<'a, str>>,
    img_instructions: Option<Cow<'a, str>>,
}

impl NormalCaptchaBuilder<'_, NoBodyProvided> {
    pub const fn new() -> Self {
        Self {
            body: NoBodyProvided,
            phrase: false,
            case: false,
            numeric: AnswerType::NoPreference,
            math: false,
            min_length: 0,
            max_length: 0,
            comment: None,
            img_instructions: None,
        }
    }
}

impl Default for NormalCaptchaBuilder<'_, NoBodyProvided> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> NormalCaptchaBuilder<'a, Body<'a>> {
    pub fn build(self) -> NormalCaptcha<'a> {
        NormalCaptcha {
            body: self.body.0,
            phrase: self.phrase,
            case: self.case,
            numeric: self.numeric,
            math: self.math,
            min_length: self.min_length,
            max_length: self.max_length,
            comment: self.comment,
            img_instructions: self.img_instructions,
        }
    }
}

impl<'a, T> NormalCaptchaBuilder<'a, T> {
    pub fn body(self, body: impl Into<Cow<'a, str>>) -> NormalCaptchaBuilder<'a, Body<'a>> {
        NormalCaptchaBuilder {
            body: Body(body.into()),
            phrase: self.phrase,
            case: self.case,
            numeric: self.numeric,
            math: self.math,
            min_length: self.min_length,
            max_length: self.max_length,
            comment: self.comment,
            img_instructions: self.img_instructions,
        }
    }

    pub fn phrase(mut self, phrase: bool) -> NormalCaptchaBuilder<'a, T> {
        self.phrase = phrase;
        self
    }

    pub fn case(mut self, case: bool) -> NormalCaptchaBuilder<'a, T> {
        self.case = case;
        self
    }

    pub fn numeric(mut self, numeric: AnswerType) -> NormalCaptchaBuilder<'a, T> {
        self.numeric = numeric;
        self
    }

    pub fn math(mut self, math: bool) -> NormalCaptchaBuilder<'a, T> {
        self.math = math;
        self
    }

    pub fn min_length(mut self, min_length: u32) -> NormalCaptchaBuilder<'a, T> {
        self.min_length = min_length;
        self
    }

    pub fn max_length(mut self, max_length: u32) -> NormalCaptchaBuilder<'a, T> {
        self.max_length = max_length;
        self
    }

    pub fn comment(
        mut self,
        comment: Option<impl Into<Cow<'a, str>>>,
    ) -> NormalCaptchaBuilder<'a, T> {
        self.comment = comment.map(Into::into);
        self
    }

    pub fn img_instructions(
        mut self,
        img_instructions: Option<impl Into<Cow<'a, str>>>,
    ) -> NormalCaptchaBuilder<'a, T> {
        self.img_instructions = img_instructions.map(Into::into);
        self
    }
}
