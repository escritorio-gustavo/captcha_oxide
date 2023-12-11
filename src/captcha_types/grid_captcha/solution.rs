use captcha_oxide_derive::captcha_solution;

#[captcha_solution]
pub struct GridCaptchaSolution {
    pub click: Box<[u8]>,
}
