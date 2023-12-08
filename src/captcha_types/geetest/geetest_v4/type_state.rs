use std::borrow::Cow;

pub struct UrlMissing;
pub struct UrlProvided<'a>(pub &'a str);

pub struct GtMissing;
pub struct GtProvided<'a>(pub Cow<'a, str>);

pub struct ChallengeMissing;
pub struct ChallengeProvided<'a>(pub Cow<'a, str>);

pub struct CaptchaIdMissing;
pub struct CaptchaIdProvided<'a>(pub Cow<'a, str>);
