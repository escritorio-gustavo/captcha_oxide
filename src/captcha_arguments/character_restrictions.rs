use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub enum CharacterRestrictions {
    #[default]
    NotSpecified,
    OnlyNumbers,
    OnlyLetters,
    OnlyNumbersOrOnlyLetters,
    BothNumbersAndLetters,
}
