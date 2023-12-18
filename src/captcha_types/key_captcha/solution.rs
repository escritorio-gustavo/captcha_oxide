use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
pub struct KeyCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}
