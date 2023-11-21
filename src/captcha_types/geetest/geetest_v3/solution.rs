use std::borrow::Cow;

use captcha_oxide_derive::captcha_solution;

#[captcha_solution]
#[serde(rename_all = "camelCase")]
pub struct GeeTestV3Solution<'a> {
    pub challenge: Cow<'a, str>,
    pub validate: Cow<'a, str>,
    pub seccode: Cow<'a, str>,
}
