use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
pub struct CutCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}
