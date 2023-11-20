use std::borrow::Cow;

use catptcha_oxide_derive::captcha_solution;

#[captcha_solution]
pub struct GeeTestV4Solution<'a> {
    pub captcha_id: Cow<'a, str>,
    pub lot_number: Cow<'a, str>,
    pub pass_token: Cow<'a, str>,
    pub gen_time: Cow<'a, str>,
    pub captcha_output: Cow<'a, str>,
}
