use captcha_oxide_derive::captcha_solution;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundingBox {
    pub x_min: u16,
    pub y_min: u16,
    pub x_max: u16,
    pub y_max: u16,
}

#[captcha_solution]
pub struct BoundingBoxCaptchaSolution {
    pub bounding_boxes: Box<[Box<[BoundingBox]>]>,
}
