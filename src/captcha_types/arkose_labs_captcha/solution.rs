use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArkoseLabsCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}
