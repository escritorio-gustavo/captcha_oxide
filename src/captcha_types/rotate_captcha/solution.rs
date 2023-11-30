use captcha_oxide_derive::captcha_solution;

#[captcha_solution]
pub struct RotateCaptchaSolution {
    pub rotate: u16,
}
