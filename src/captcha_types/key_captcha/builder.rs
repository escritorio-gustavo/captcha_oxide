use std::borrow::Cow;

use url::Url;

use crate::{
    prelude::*,
    proxy::Proxy,
    type_state::{UrlMissing, UrlProvided},
};

use super::{
    type_state::{
        SessionIdMissing, SessionIdProvided, UserIdMissing, UserIdProvided, WebServerSign2Missing,
        WebServerSign2Provided, WebServerSignMissing, WebServerSignProvided,
    },
    KeyCaptcha,
};

pub struct KeyCaptchaBuilder<'a, T, U, V, W, X> {
    website_url: T,
    user_id: U,
    session_id: V,
    web_server_sign: W,
    web_server_sign2: X,
    proxy: Option<Proxy<'a>>,
}

impl<'a>
    KeyCaptchaBuilder<
        'a,
        UrlProvided<'a>,
        UserIdProvided,
        SessionIdProvided<'a>,
        WebServerSignProvided<'a>,
        WebServerSign2Provided<'a>,
    >
{
    pub fn build(self) -> Result<KeyCaptcha<'a>> {
        Ok(KeyCaptcha {
            website_url: Url::parse(self.website_url.0)?,
            user_id: self.user_id.0,
            session_id: self.session_id.0,
            web_server_sign: self.web_server_sign.0,
            web_server_sign2: self.web_server_sign2.0,
            task_type: self.proxy.into(),
        })
    }
}

impl<'a>
    KeyCaptchaBuilder<
        'a,
        UrlMissing,
        UserIdMissing,
        SessionIdMissing,
        WebServerSignMissing,
        WebServerSign2Missing,
    >
{
    pub const fn new() -> Self {
        Self {
            website_url: UrlMissing,
            user_id: UserIdMissing,
            session_id: SessionIdMissing,
            web_server_sign: WebServerSignMissing,
            web_server_sign2: WebServerSign2Missing,
            proxy: None,
        }
    }
}

impl<'a> Default
    for KeyCaptchaBuilder<
        'a,
        UrlMissing,
        UserIdMissing,
        SessionIdMissing,
        WebServerSignMissing,
        WebServerSign2Missing,
    >
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, U, V, W, X> KeyCaptchaBuilder<'a, T, U, V, W, X> {
    pub fn website_url(
        self,
        website_url: &'a str,
    ) -> KeyCaptchaBuilder<'a, UrlProvided<'a>, U, V, W, X> {
        KeyCaptchaBuilder {
            website_url: UrlProvided(website_url),
            user_id: self.user_id,
            session_id: self.session_id,
            web_server_sign: self.web_server_sign,
            web_server_sign2: self.web_server_sign2,
            proxy: self.proxy,
        }
    }

    pub fn user_id(self, user_id: u32) -> KeyCaptchaBuilder<'a, T, UserIdProvided, V, W, X> {
        KeyCaptchaBuilder {
            website_url: self.website_url,
            user_id: UserIdProvided(user_id),
            session_id: self.session_id,
            web_server_sign: self.web_server_sign,
            web_server_sign2: self.web_server_sign2,
            proxy: self.proxy,
        }
    }

    pub fn session_id(
        self,
        session_id: impl Into<Cow<'a, str>>,
    ) -> KeyCaptchaBuilder<'a, T, U, SessionIdProvided<'a>, W, X> {
        KeyCaptchaBuilder {
            website_url: self.website_url,
            user_id: self.user_id,
            session_id: SessionIdProvided(session_id.into()),
            web_server_sign: self.web_server_sign,
            web_server_sign2: self.web_server_sign2,
            proxy: self.proxy,
        }
    }

    pub fn web_server_sign(
        self,
        web_server_sign: impl Into<Cow<'a, str>>,
    ) -> KeyCaptchaBuilder<'a, T, U, V, WebServerSignProvided<'a>, X> {
        KeyCaptchaBuilder {
            website_url: self.website_url,
            user_id: self.user_id,
            session_id: self.session_id,
            web_server_sign: WebServerSignProvided(web_server_sign.into()),
            web_server_sign2: self.web_server_sign2,
            proxy: self.proxy,
        }
    }

    pub fn web_server_sign2(
        self,
        web_server_sign2: impl Into<Cow<'a, str>>,
    ) -> KeyCaptchaBuilder<'a, T, U, V, W, WebServerSign2Provided<'a>> {
        KeyCaptchaBuilder {
            website_url: self.website_url,
            user_id: self.user_id,
            session_id: self.session_id,
            web_server_sign: self.web_server_sign,
            web_server_sign2: WebServerSign2Provided(web_server_sign2.into()),
            proxy: self.proxy,
        }
    }

    pub fn proxy(mut self, proxy: Option<Proxy<'a>>) -> Self {
        self.proxy = proxy;
        self
    }
}
