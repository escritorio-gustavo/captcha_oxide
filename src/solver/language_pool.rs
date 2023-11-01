#[derive(Debug, Default, Clone, Copy, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LanguagePool {
    #[default]
    En,
    Ru,
}
