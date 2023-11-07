use std::borrow::Cow;

use crate::{
    captcha_types::geetest::type_state::{
        ChallengeMissing, ChallengeProvided, GtMissing, GtProvided,
    },
    prelude::*,
    proxy::Proxy,
    type_state::{UrlMissing, UrlProvided},
};

use super::GeeTestV3;

pub struct GeeTestV3Builder<'a, T, U, V> {
    website_url: T,
    gt: U,
    challenge: V,
    geetest_api_server_subdomain: Option<Cow<'a, str>>,
    user_agent: Option<Cow<'a, str>>,
    proxy: Option<Proxy<'a>>,
}

impl<'a> GeeTestV3Builder<'a, UrlProvided, GtProvided<'a>, ChallengeProvided<'a>> {
    pub fn build(self) -> GeeTestV3<'a> {
        GeeTestV3 {
            task_type: self.proxy.into(),
            website_url: self.website_url.0,
            gt: self.gt.0,
            challenge: self.challenge.0,
            geetest_api_server_subdomain: self.geetest_api_server_subdomain,
            user_agent: self.user_agent,
            version: 3,
        }
    }
}

impl<'a> GeeTestV3Builder<'a, UrlMissing, GtMissing, ChallengeMissing> {
    pub const fn new() -> Self {
        Self {
            website_url: UrlMissing,
            gt: GtMissing,
            challenge: ChallengeMissing,
            geetest_api_server_subdomain: None,
            user_agent: None,
            proxy: None,
        }
    }
}

impl<'a> Default for GeeTestV3Builder<'a, UrlMissing, GtMissing, ChallengeMissing> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, U, V> GeeTestV3Builder<'a, T, U, V> {
    pub fn website_url(self, website_url: &str) -> Result<GeeTestV3Builder<'a, UrlProvided, U, V>> {
        Ok(GeeTestV3Builder {
            website_url: UrlProvided(url::Url::parse(website_url)?),
            gt: self.gt,
            challenge: self.challenge,
            geetest_api_server_subdomain: self.geetest_api_server_subdomain,
            user_agent: self.user_agent,
            proxy: self.proxy,
        })
    }

    pub fn gt(self, gt: impl Into<Cow<'a, str>>) -> GeeTestV3Builder<'a, T, GtProvided<'a>, V> {
        GeeTestV3Builder {
            website_url: self.website_url,
            gt: GtProvided(gt.into()),
            challenge: self.challenge,
            geetest_api_server_subdomain: self.geetest_api_server_subdomain,
            user_agent: self.user_agent,
            proxy: self.proxy,
        }
    }

    pub fn challenge(
        self,
        challenge: impl Into<Cow<'a, str>>,
    ) -> GeeTestV3Builder<'a, T, U, ChallengeProvided<'a>> {
        GeeTestV3Builder {
            website_url: self.website_url,
            gt: self.gt,
            challenge: ChallengeProvided(challenge.into()),
            geetest_api_server_subdomain: self.geetest_api_server_subdomain,
            user_agent: self.user_agent,
            proxy: self.proxy,
        }
    }

    pub fn geetest_api_server_subdomain(
        mut self,
        geetest_api_server_subdomain: Option<impl Into<Cow<'a, str>>>,
    ) -> Self {
        self.geetest_api_server_subdomain = geetest_api_server_subdomain.map(Into::into);
        self
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
