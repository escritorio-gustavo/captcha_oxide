use std::borrow::Cow;

use crate::{
    prelude::*,
    proxy::Proxy,
    type_state::{UrlMissing, UrlProvided, WebsitePublicKeyMissing, WebsitePublicKeyProvided},
};

use super::ArkoseLabsCaptcha;

pub struct ArkoseLabsCaptchaBuilder<'a, T, U, V>
where
    V: serde::Serialize,
{
    pub(super) website_url: T,
    pub(super) website_public_key: U,

    pub(super) funcaptcha_api_jssubdomain: Option<Cow<'a, str>>,

    pub(super) data: Option<V>,

    pub(super) user_agent: Option<Cow<'a, str>>,
    pub(super) proxy: Option<Proxy<'a>>,
}

impl<'a, T> ArkoseLabsCaptchaBuilder<'a, UrlMissing, WebsitePublicKeyMissing, T>
where
    T: serde::Serialize,
{
    pub const fn new() -> Self {
        Self {
            website_url: UrlMissing,
            website_public_key: WebsitePublicKeyMissing,
            funcaptcha_api_jssubdomain: None,
            data: None,
            user_agent: None,
            proxy: None,
        }
    }
}

impl<'a, T> Default for ArkoseLabsCaptchaBuilder<'a, UrlMissing, WebsitePublicKeyMissing, T>
where
    T: serde::Serialize,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> ArkoseLabsCaptchaBuilder<'a, UrlProvided<'a>, WebsitePublicKeyProvided<'a>, T>
where
    T: serde::Serialize,
{
    pub fn build(self) -> Result<ArkoseLabsCaptcha<'a, T>> {
        Ok(ArkoseLabsCaptcha {
            _x: std::marker::PhantomData,
            task_type: self.proxy.into(),
            website_url: url::Url::parse(self.website_url.0)?,
            website_public_key: self.website_public_key.0,
            funcaptcha_api_jssubdomain: self.funcaptcha_api_jssubdomain,
            data: self
                .data
                .map(|x| serde_json::to_string(&x))
                .transpose()?
                .map(Into::into),
            user_agent: self.user_agent,
        })
    }
}

impl<'a, T, U, V> ArkoseLabsCaptchaBuilder<'a, T, U, V>
where
    V: serde::Serialize,
{
    pub fn website_url(self, website_url: &str) -> ArkoseLabsCaptchaBuilder<'a, UrlProvided, U, V> {
        ArkoseLabsCaptchaBuilder {
            website_url: UrlProvided(website_url),
            website_public_key: self.website_public_key,
            funcaptcha_api_jssubdomain: self.funcaptcha_api_jssubdomain,
            data: self.data,
            user_agent: self.user_agent,
            proxy: self.proxy,
        }
    }

    pub fn website_public_key(
        self,
        website_public_key: impl Into<Cow<'a, str>>,
    ) -> ArkoseLabsCaptchaBuilder<'a, T, WebsitePublicKeyProvided<'a>, V> {
        ArkoseLabsCaptchaBuilder {
            website_url: self.website_url,
            website_public_key: WebsitePublicKeyProvided(website_public_key.into()),
            funcaptcha_api_jssubdomain: self.funcaptcha_api_jssubdomain,
            data: self.data,
            user_agent: self.user_agent,
            proxy: self.proxy,
        }
    }

    pub fn funcaptcha_api_jssubdomain(
        mut self,
        funcaptcha_api_jssubdomain: Option<impl Into<Cow<'a, str>>>,
    ) -> Self {
        self.funcaptcha_api_jssubdomain = funcaptcha_api_jssubdomain.map(Into::into);
        self
    }

    pub fn data(mut self, data: Option<V>) -> Self {
        self.data = data;
        self
    }

    pub fn user_agent(mut self, user_agent: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.user_agent = user_agent.map(Into::into);
        self
    }

    pub fn proxy(mut self, proxy: Option<Proxy<'a>>) -> Self {
        self.proxy = proxy.map(Into::into);
        self
    }
}
