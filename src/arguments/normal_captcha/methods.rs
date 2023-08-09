use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NormalCaptchaMethods {
    Post {
        bytes: Vec<u8>,
        mime_str: String,
        file_extension: String,
    },
    Base64(String),
}
