#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundingBox {
    pub x_min: u16,
    pub y_min: u16,
    pub x_max: u16,
    pub y_max: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct BoundingBoxCaptchaSolution {
    pub bounding_boxes: Box<[Box<[BoundingBox]>]>,
}
