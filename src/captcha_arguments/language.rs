use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Language {
    #[default]
    NotSpecified,
    Cyrillic,
    Latin,
}
