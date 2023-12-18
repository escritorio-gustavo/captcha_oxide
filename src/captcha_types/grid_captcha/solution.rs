#[derive(serde::Deserialize, Debug)]
pub struct GridCaptchaSolution {
    pub click: Box<[u8]>,
}
