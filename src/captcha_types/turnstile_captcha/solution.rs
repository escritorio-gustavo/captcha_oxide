use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
pub struct TurnstileCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
    pub user_agent: Cow<'a, str>,
}
