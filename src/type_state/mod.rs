use std::borrow::Cow;

pub struct UrlMissing;
pub struct UrlProvided(pub url::Url);

pub struct WebsiteKeyMissing;
pub struct WebsiteKeyProvided<'a>(pub Cow<'a, str>);

pub struct WebsitePublicKeyMissing;
pub struct WebsitePublicKeyProvided<'a>(pub Cow<'a, str>);

pub struct MinScoreMissing;
pub struct MinScoreProvided(pub f32);
