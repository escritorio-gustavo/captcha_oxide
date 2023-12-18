use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaSolution<'a> {
    pub g_recaptcha_response: Cow<'a, str>,
    pub token: Cow<'a, str>,
}
