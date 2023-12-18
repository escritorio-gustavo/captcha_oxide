use std::borrow::Cow;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
    pub resp_key: Cow<'a, str>,
    pub user_agent: Cow<'a, str>,
    pub g_recaptcha_response: Cow<'a, str>,
}
