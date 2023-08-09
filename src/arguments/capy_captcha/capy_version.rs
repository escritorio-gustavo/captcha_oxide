use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub enum CapyVersion {
    #[default]
    Puzzle,
    Avatar,
}

impl ToString for CapyVersion {
    fn to_string(&self) -> String {
        match self {
            CapyVersion::Puzzle => "puzzle",
            CapyVersion::Avatar => "avatar",
        }
        .into()
    }
}
