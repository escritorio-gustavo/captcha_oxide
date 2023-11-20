use std::borrow::Cow;

pub struct UserIdMissing;
pub struct UserIdProvided(pub u32);

pub struct SessionIdMissing;
pub struct SessionIdProvided<'a>(pub Cow<'a, str>);

pub struct WebServerSignMissing;
pub struct WebServerSignProvided<'a>(pub Cow<'a, str>);

pub struct WebServerSign2Missing;
pub struct WebServerSign2Provided<'a>(pub Cow<'a, str>);
