use std::borrow::Cow;

pub struct CaptchaIdMissing;
pub struct CaptchaIdProvided<'a>(pub Cow<'a, str>);

pub struct DivIdMissing;
pub struct DivIdProvided<'a>(pub Cow<'a, str>);
