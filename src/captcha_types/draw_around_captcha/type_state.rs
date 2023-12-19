use std::borrow::Cow;

pub struct BodyMissing;
pub struct BodyProvided<'a>(pub Cow<'a, str>);

pub struct CommentMissing;
pub struct CommentProvided<'a>(pub Cow<'a, str>);

pub struct ImgInstructionsMissing;
pub struct ImgInstructionsProvided<'a>(pub Cow<'a, str>);
