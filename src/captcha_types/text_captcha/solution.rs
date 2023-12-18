use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
pub struct TextCaptchaSolution<'a> {
    pub text: Cow<'a, str>,
}
