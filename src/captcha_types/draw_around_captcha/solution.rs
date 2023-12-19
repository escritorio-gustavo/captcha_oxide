#[derive(Debug, serde::Deserialize)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct DrawAroundCaptchaSolution {
    pub canvas: Box<[Box<[Coordinates]>]>,
}
