use std::borrow::Cow;

use crate::{
    prelude::*,
    proxy::Proxy,
    type_state::{UrlMissing, UrlProvided, WebsiteKeyMissing, WebsiteKeyProvided},
};

use super::TurnstileStandaloneCaptcha;

pub struct TurnstileStandaloneCaptchaBuilder<'a, T, U> {
    website_url: T,
    website_key: U,
    user_agent: Option<Cow<'a, str>>,
    proxy: Option<Proxy<'a>>,
}

impl<'a> TurnstileStandaloneCaptchaBuilder<'a, UrlProvided<'a>, WebsiteKeyProvided<'a>> {
    pub fn build(self) -> Result<TurnstileStandaloneCaptcha<'a>> {
        Ok(TurnstileStandaloneCaptcha {
            task_type: self.proxy.into(),
            website_url: url::Url::parse(self.website_url.0)?,
            website_key: self.website_key.0,
            user_agent: self.user_agent,
        })
    }
}

impl<'a> TurnstileStandaloneCaptchaBuilder<'a, UrlMissing, WebsiteKeyMissing> {
    pub const fn new() -> Self {
        Self {
            website_url: UrlMissing,
            website_key: WebsiteKeyMissing,
            user_agent: None,
            proxy: None,
        }
    }
}

impl<'a> Default for TurnstileStandaloneCaptchaBuilder<'a, UrlMissing, WebsiteKeyMissing> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, U> TurnstileStandaloneCaptchaBuilder<'a, T, U> {
    pub fn website_url(
        self,
        website_url: &str,
    ) -> TurnstileStandaloneCaptchaBuilder<'a, UrlProvided, U> {
        TurnstileStandaloneCaptchaBuilder {
            website_url: UrlProvided(website_url),
            website_key: self.website_key,
            user_agent: self.user_agent,
            proxy: self.proxy,
        }
    }

    pub fn website_key(
        self,
        website_key: impl Into<Cow<'a, str>>,
    ) -> TurnstileStandaloneCaptchaBuilder<'a, T, WebsiteKeyProvided<'a>> {
        TurnstileStandaloneCaptchaBuilder {
            website_url: self.website_url,
            website_key: WebsiteKeyProvided(website_key.into()),
            user_agent: self.user_agent,
            proxy: self.proxy,
        }
    }

    pub fn user_agent(mut self, user_agent: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.user_agent = user_agent.map(Into::into);
        self
    }

    pub fn proxy(mut self, proxy: Option<Proxy<'a>>) -> Self {
        self.proxy = proxy;
        self
    }
}
