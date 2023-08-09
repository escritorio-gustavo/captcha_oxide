use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
/// What types of characters are allowed on the captcha response
pub enum CharacterRestrictions {
    #[default]
    NotSpecified,
    OnlyNumbers,
    OnlyLetters,
    OnlyNumbersOrOnlyLetters,
    BothNumbersAndLetters,
}
