use std::borrow::Cow;

use captcha_oxide_derive::captcha_solution;

#[captcha_solution]
#[serde(rename_all = "camelCase")]
pub struct GeeTestV4Solution<'a> {
    pub captcha_id: Cow<'a, str>,
    pub lot_number: Cow<'a, str>,
    pub pass_token: Cow<'a, str>,
    pub gen_time: Cow<'a, str>,
    pub captcha_output: Cow<'a, str>,
}
