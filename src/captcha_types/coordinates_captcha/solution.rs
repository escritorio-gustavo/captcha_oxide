#[derive(Debug, serde::Deserialize)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct CoordinatesCaptchaSolution {
    pub coordinates: Box<[Coordinates]>,
}
