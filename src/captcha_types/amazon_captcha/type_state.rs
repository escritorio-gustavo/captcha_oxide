use std::borrow::Cow;

pub struct IvMissing;
pub struct IvProvided<'a>(pub Cow<'a, str>);

pub struct ContextMissing;
pub struct ContextProvided<'a>(pub Cow<'a, str>);
