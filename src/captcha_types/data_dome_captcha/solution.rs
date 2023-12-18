use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
pub struct DataDomeCaptchaSolution<'a> {
    pub cookie: Cow<'a, str>,
}
