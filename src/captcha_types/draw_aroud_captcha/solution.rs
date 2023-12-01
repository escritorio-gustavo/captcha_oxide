use captcha_oxide_derive::captcha_solution;

#[derive(Debug, serde::Deserialize)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

#[captcha_solution]
pub struct DrawAroundCaptchaSolution {
    pub canvas: Box<[Box<[Coordinates]>]>,
}
