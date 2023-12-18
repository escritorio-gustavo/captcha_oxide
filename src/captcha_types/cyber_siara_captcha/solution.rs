use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
pub struct CyberSiARACaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}
