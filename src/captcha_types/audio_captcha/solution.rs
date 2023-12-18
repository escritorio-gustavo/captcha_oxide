use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
pub struct AudioCaptchaSolution<'a> {
    pub text: Cow<'a, str>,
}
