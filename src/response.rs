use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct CaptchaResponse {
    pub(crate) status: u8,
    pub(crate) request: String,
    pub(crate) error_text: Option<String>,
}
