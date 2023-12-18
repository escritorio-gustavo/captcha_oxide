use super::{language_pool::LanguagePool, CaptchaSolver};
use url::Url;

pub struct NoApiKeyProvided;
pub struct ApiKey(Box<str>);

pub struct SolverBuilder<T> {
    api_key: T,
    language_pool: LanguagePool,
    callback_url: Option<Url>,
}

impl SolverBuilder<NoApiKeyProvided> {
    pub const fn new() -> Self {
        Self {
            api_key: NoApiKeyProvided,
            language_pool: LanguagePool::En,
            callback_url: None,
        }
    }
}

impl Default for SolverBuilder<NoApiKeyProvided> {
    fn default() -> Self {
        Self::new()
    }
}

impl SolverBuilder<ApiKey> {
    pub fn build(self) -> CaptchaSolver {
        CaptchaSolver {
            api_key: self.api_key.0,
            language_pool: self.language_pool,
            callback_url: self.callback_url,
        }
    }
}

impl<T> SolverBuilder<T> {
    pub fn api_key(self, api_key: impl Into<Box<str>>) -> SolverBuilder<ApiKey> {
        SolverBuilder {
            api_key: ApiKey(api_key.into()),
            language_pool: self.language_pool,
            callback_url: self.callback_url,
        }
    }

    pub fn language_pool(mut self, language_pool: LanguagePool) -> Self {
        self.language_pool = language_pool;
        self
    }

    pub fn callback_url(mut self, callback_url: Option<Url>) -> Self {
        self.callback_url = callback_url;
        self
    }
}
