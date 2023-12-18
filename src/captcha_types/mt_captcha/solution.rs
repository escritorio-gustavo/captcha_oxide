use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
pub struct MtCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}
