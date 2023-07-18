use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub enum Language {
    #[default]
    NotSpecified,
    Cyrillic,
    Latin,
}
