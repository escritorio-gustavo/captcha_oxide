use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
pub struct NormalCaptchaSolution<'a> {
    pub text: Cow<'a, str>,
}
