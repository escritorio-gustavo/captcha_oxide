use std::borrow::Cow;

use crate::{
    prelude::*,
    proxy::Proxy,
    type_state::{UrlMissing, UrlProvided},
};

use super::{
    type_state::{CaptchaIdMissing, CaptchaIdProvided, DivIdMissing, DivIdProvided},
    LeminCaptcha,
};

pub struct LeminCaptchaBuilder<'a, T, U, V> {
    website_url: T,
    captcha_id: U,
    div_id: V,
    lemin_api_server_subdomain: Option<Cow<'a, str>>,
    user_agent: Option<Cow<'a, str>>,
    proxy: Option<Proxy<'a>>,
}

impl<'a> LeminCaptchaBuilder<'a, UrlProvided<'a>, CaptchaIdProvided<'a>, DivIdProvided<'a>> {
    pub fn build(self) -> Result<LeminCaptcha<'a>> {
        Ok(LeminCaptcha {
            website_url: url::Url::parse(self.website_url.0)?,
            captcha_id: self.captcha_id.0,
            div_id: self.div_id.0,
            lemin_api_server_subdomain: self.lemin_api_server_subdomain,
            user_agent: self.user_agent,
            task_type: self.proxy.into(),
        })
    }
}

impl<'a> LeminCaptchaBuilder<'a, UrlMissing, CaptchaIdMissing, DivIdMissing> {
    pub const fn new() -> Self {
        Self {
            website_url: UrlMissing,
            captcha_id: CaptchaIdMissing,
            div_id: DivIdMissing,
            lemin_api_server_subdomain: None,
            user_agent: None,
            proxy: None,
        }
    }
}

impl<'a> Default for LeminCaptchaBuilder<'a, UrlMissing, CaptchaIdMissing, DivIdMissing> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, U, V> LeminCaptchaBuilder<'a, T, U, V> {
    pub fn website_url(
        self,
        website_url: &'a str,
    ) -> LeminCaptchaBuilder<'a, UrlProvided<'a>, U, V> {
        LeminCaptchaBuilder {
            website_url: UrlProvided(website_url),
            captcha_id: self.captcha_id,
            div_id: self.div_id,
            lemin_api_server_subdomain: self.lemin_api_server_subdomain,
            user_agent: self.user_agent,
            proxy: self.proxy,
        }
    }

    pub fn captcha_id(
        self,
        captcha_id: impl Into<Cow<'a, str>>,
    ) -> LeminCaptchaBuilder<'a, T, CaptchaIdProvided<'a>, V> {
        LeminCaptchaBuilder {
            website_url: self.website_url,
            captcha_id: CaptchaIdProvided(captcha_id.into()),
            div_id: self.div_id,
            lemin_api_server_subdomain: self.lemin_api_server_subdomain,
            user_agent: self.user_agent,
            proxy: self.proxy,
        }
    }

    pub fn div_id(
        self,
        div_id: impl Into<Cow<'a, str>>,
    ) -> LeminCaptchaBuilder<'a, T, U, DivIdProvided<'a>> {
        LeminCaptchaBuilder {
            website_url: self.website_url,
            captcha_id: self.captcha_id,
            div_id: DivIdProvided(div_id.into()),
            lemin_api_server_subdomain: self.lemin_api_server_subdomain,
            user_agent: self.user_agent,
            proxy: self.proxy,
        }
    }

    pub fn lemin_api_server_subdomain(
        mut self,
        lemin_api_server_subdomain: Option<impl Into<Cow<'a, str>>>,
    ) -> Self {
        self.lemin_api_server_subdomain = lemin_api_server_subdomain.map(Into::into);
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
