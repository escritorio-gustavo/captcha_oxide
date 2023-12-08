use std::borrow::Cow;

use super::{type_state::*, GeeTestV4, InitParameters};
use crate::{prelude::*, proxy::Proxy};

pub struct GeeTestV4Builder<'a, T, U, V, W, X>
where
    X: serde::Serialize,
{
    website_url: T,
    gt: U,
    challenge: V,
    captcha_id: W,
    geetest_api_server_subdomain: Option<Cow<'a, str>>,
    user_agent: Option<Cow<'a, str>>,
    init_parameters_data: Option<X>,
    proxy: Option<Proxy<'a>>,
}

impl<'a, T>
    GeeTestV4Builder<
        'a,
        UrlProvided<'a>,
        GtProvided<'a>,
        ChallengeProvided<'a>,
        CaptchaIdProvided<'a>,
        T,
    >
where
    T: serde::Serialize,
{
    pub fn build(self) -> Result<GeeTestV4<'a, T>> {
        Ok(GeeTestV4 {
            task_type: self.proxy.into(),
            website_url: url::Url::parse(self.website_url.0)?,
            gt: self.gt.0,
            challenge: self.challenge.0,
            geetest_api_server_subdomain: self.geetest_api_server_subdomain,
            user_agent: self.user_agent,
            init_parameters: InitParameters {
                captcha_id: self.captcha_id.0,
                data: self.init_parameters_data,
            },
            version: 4,
        })
    }
}

impl<'a, T> GeeTestV4Builder<'a, UrlMissing, GtMissing, ChallengeMissing, CaptchaIdMissing, T>
where
    T: serde::Serialize,
{
    pub const fn new() -> Self {
        Self {
            proxy: None,
            website_url: UrlMissing,
            gt: GtMissing,
            challenge: ChallengeMissing,
            captcha_id: CaptchaIdMissing,
            geetest_api_server_subdomain: None,
            user_agent: None,
            init_parameters_data: None,
        }
    }
}

impl<'a, T> Default
    for GeeTestV4Builder<'a, UrlMissing, GtMissing, ChallengeMissing, CaptchaIdMissing, T>
where
    T: serde::Serialize,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, U, V, W, X> GeeTestV4Builder<'a, T, U, V, W, X>
where
    X: serde::Serialize,
{
    pub fn website_url(self, website_url: &str) -> GeeTestV4Builder<'a, UrlProvided, U, V, W, X> {
        GeeTestV4Builder {
            website_url: UrlProvided(website_url),
            gt: self.gt,
            challenge: self.challenge,
            captcha_id: self.captcha_id,
            geetest_api_server_subdomain: self.geetest_api_server_subdomain,
            user_agent: self.user_agent,
            init_parameters_data: self.init_parameters_data,
            proxy: self.proxy,
        }
    }

    pub fn gt(
        self,
        gt: impl Into<Cow<'a, str>>,
    ) -> GeeTestV4Builder<'a, T, GtProvided<'a>, V, W, X> {
        GeeTestV4Builder {
            website_url: self.website_url,
            gt: GtProvided(gt.into()),
            challenge: self.challenge,
            captcha_id: self.captcha_id,
            geetest_api_server_subdomain: self.geetest_api_server_subdomain,
            user_agent: self.user_agent,
            init_parameters_data: self.init_parameters_data,
            proxy: self.proxy,
        }
    }

    pub fn challenge(
        self,
        challenge: impl Into<Cow<'a, str>>,
    ) -> GeeTestV4Builder<'a, T, U, ChallengeProvided<'a>, W, X> {
        GeeTestV4Builder {
            website_url: self.website_url,
            gt: self.gt,
            challenge: ChallengeProvided(challenge.into()),
            captcha_id: self.captcha_id,
            geetest_api_server_subdomain: self.geetest_api_server_subdomain,
            user_agent: self.user_agent,
            init_parameters_data: self.init_parameters_data,
            proxy: self.proxy,
        }
    }

    pub fn captcha_id(
        self,
        captcha_id: impl Into<Cow<'a, str>>,
    ) -> GeeTestV4Builder<'a, T, U, V, CaptchaIdProvided<'a>, X> {
        GeeTestV4Builder {
            website_url: self.website_url,
            gt: self.gt,
            challenge: self.challenge,
            captcha_id: CaptchaIdProvided(captcha_id.into()),
            geetest_api_server_subdomain: self.geetest_api_server_subdomain,
            user_agent: self.user_agent,
            init_parameters_data: self.init_parameters_data,
            proxy: self.proxy,
        }
    }

    pub fn geetest_api_server_subdomain(
        mut self,
        geetest_api_server_domain: Option<impl Into<Cow<'a, str>>>,
    ) -> Self {
        self.geetest_api_server_subdomain = geetest_api_server_domain.map(Into::into);
        self
    }

    pub fn user_agent(mut self, user_agent: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.user_agent = user_agent.map(Into::into);
        self
    }

    pub fn init_parameters_data(mut self, init_parameters_data: Option<X>) -> Self {
        self.init_parameters_data = init_parameters_data;
        self
    }

    pub fn proxy(mut self, proxy: Option<Proxy<'a>>) -> Self {
        self.proxy = proxy;
        self
    }
}
