use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeeTestV3Solution<'a> {
    pub challenge: Cow<'a, str>,
    pub validate: Cow<'a, str>,
    pub seccode: Cow<'a, str>,
}
