use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ImageInstructions {
    pub bytes: Vec<u8>,
    pub mime_type: String,
    pub extension: String,
}
