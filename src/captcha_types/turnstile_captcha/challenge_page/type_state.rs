use std::borrow::Cow;

pub struct UserAgentMissing;
pub struct UserAgentProvided<'a>(pub Cow<'a, str>);

pub struct ActionMissing;
pub struct ActionProvided<'a>(pub Cow<'a, str>);

pub struct DataMissing;
pub struct DataProvided<'a>(pub Cow<'a, str>);

pub struct PageDataMissing;
pub struct PageDataProvided<'a>(pub Cow<'a, str>);
