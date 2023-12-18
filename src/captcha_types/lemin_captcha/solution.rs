use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
pub struct LeminCaptchaSolution<'a> {
    pub answer: Cow<'a, str>,

    pub challenge_id: Cow<'a, str>,
}
