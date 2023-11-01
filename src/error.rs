use crate::solver::SolveError;

#[derive(thiserror::Error, serde::Serialize, Debug)]
pub enum Error {
    #[error(transparent)]
    #[serde(serialize_with = "serialize_error")]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    #[serde(serialize_with = "serialize_error")]
    HttpError(#[from] reqwest::Error),

    #[error(transparent)]
    #[serde(serialize_with = "serialize_error")]
    TwoCaptchaError(#[from] SolveError),
}

fn serialize_error<S: serde::Serializer>(
    v: &impl std::error::Error,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(v.to_string().as_ref())
}
