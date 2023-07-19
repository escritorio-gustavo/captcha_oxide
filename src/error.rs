use thiserror;

#[derive(Debug, thiserror::Error)]
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
    UrlParseError(#[from] url::ParseError),

    #[error("{0}")]
    /// Failed to parse the API's JSON response
    JsonParseError(#[from] serde_json::Error),

    #[error("{0}")]
    /// Unknown network error
    ResquestFailError(#[from] reqwest::Error),

    #[error("Failed to set the file mimetype")]
    FileParseError,
}
