use std::borrow::Cow;

pub struct NoUrlProvided;
pub struct Url(pub url::Url);

pub struct NoWebsiteKeyProvided;
pub struct WebsiteKey<'a>(pub Cow<'a, str>);

pub struct NoMinScoreProvided;
pub struct MinScore(pub f32);
