use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum CharacterRestrictions {
    #[default]
    NotSpecified,
    OnlyNumbers,
    OnlyLetters,
    OnlyNumbersOrOnlyLetters,
    BothNumbersAndLetters,
}
