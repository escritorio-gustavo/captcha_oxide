use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
pub struct FriendlyCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}
