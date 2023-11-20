use std::borrow::Cow;

use catptcha_oxide_derive::captcha_solution;

#[captcha_solution]
#[serde(rename_all = "camelCase")]
pub struct ArkoseLabsCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}
