use thiserror;

#[derive(Debug, thiserror::Error, Clone, serde::Serialize)]
/// Represents the types of errors that can occur when solving a captcha
pub enum Error {
    #[error("2captcha's API returned an error: {error_code} {error_description:?}")]
    /// The 2captcha API returned an error
    TwoCaptchaError {
        error_code: String,
        error_description: Option<String>,
    },

    #[error("UrlParseError: {0}")]
    /// Failed to parse an URL
    UrlParseError(String),

    #[error("JSON parse error: {0}")]
    /// Failed to parse the API's JSON response
    JsonParseError(String),

    #[error("Unknown network error: {0}")]
    /// Unknown network error
    ResquestFailError(String),

    #[error("Failed to set the file mimetype")]
    FileParseError,
}

#[doc(hidden)]
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonParseError(value.to_string())
    }
}

#[doc(hidden)]
impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParseError(value.to_string())
    }
}

#[doc(hidden)]
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ResquestFailError(value.to_string())
    }
}
