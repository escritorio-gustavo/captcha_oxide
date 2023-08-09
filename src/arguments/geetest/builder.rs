pub use super::type_state::{
    challenge::{Challenge, ChallengeNotProvided},
    gt::{Gt, GtNotProvided},
};

use crate::arguments::{
    geetest::Geetest,
    proxy::Proxy,
    type_state::page_url::{PageUrl, PageUrlNotProvided},
};

#[derive(Default, Debug, Clone)]
/// Builds a [`Geetest`] instance using the typestate pattern
/// to help avoid sending avoid inconsistent data to the
/// 2captcha API
///
/// # Example
/// ```
/// use captcha_oxide::arguments::Geetest;
///
/// let geetest_args = Geetest::builder()
///     .page_url("SOME URL")
///     .gt("DYNAMICALLY GENERATED")
///     .challenge("DYNAMICALLY GENERATED")
///     .build();
/// ```
pub struct GeetestBuilder<G, U, C> {
    gt: G,
    page_url: U,
    challenge: C,
    api_server: Option<String>,
    offline: Option<bool>,
    new_captcha: Option<bool>,
    pingback: Option<String>,
    proxy: Option<Proxy>,
    user_agent: Option<String>,
}

impl GeetestBuilder<GtNotProvided, PageUrlNotProvided, ChallengeNotProvided> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl GeetestBuilder<Gt, PageUrl, Challenge> {
    pub fn build(self) -> Geetest {
        Geetest {
            gt: self.gt.0,
            page_url: self.page_url.0,
            challenge: self.challenge.0,
            api_server: self.api_server,
            offline: self.offline,
            new_captcha: self.new_captcha,
            pingback: self.pingback,
            proxy: self.proxy,
            user_agent: self.user_agent,
        }
    }
}

impl<G, U, C> GeetestBuilder<G, U, C> {
    /// Public website key. You should be able to find it in the page's HTML
    pub fn gt(self, gt: impl Into<String>) -> GeetestBuilder<Gt, U, C> {
        GeetestBuilder {
            gt: Gt(gt.into()),
            page_url: self.page_url,
            challenge: self.challenge,
            api_server: self.api_server,
            offline: self.offline,
            new_captcha: self.new_captcha,
            pingback: self.pingback,
            proxy: self.proxy,
            user_agent: self.user_agent,
        }
    }

    /// Full URL of the page where you see the captcha
    pub fn page_url(self, page_url: impl Into<String>) -> GeetestBuilder<G, PageUrl, C> {
        GeetestBuilder {
            gt: self.gt,
            page_url: PageUrl(page_url.into()),
            challenge: self.challenge,
            api_server: self.api_server,
            offline: self.offline,
            new_captcha: self.new_captcha,
            pingback: self.pingback,
            proxy: self.proxy,
            user_agent: self.user_agent,
        }
    }

    /// Challenge key. Warning, this field is dynamically generated, so you will
    /// need to get its value at runtime
    pub fn challenge(self, challenge: impl Into<String>) -> GeetestBuilder<G, U, Challenge> {
        GeetestBuilder {
            gt: self.gt,
            page_url: self.page_url,
            challenge: Challenge(challenge.into()),
            api_server: self.api_server,
            offline: self.offline,
            new_captcha: self.new_captcha,
            pingback: self.pingback,
            proxy: self.proxy,
            user_agent: self.user_agent,
        }
    }

    /// API domain
    pub fn api_server(mut self, api_server: impl Into<String>) -> Self {
        self.api_server = Some(api_server.into());
        self
    }

    /// In rare cases initGeetest can be called with an offline parameter
    pub fn offline(mut self, offline: bool) -> Self {
        self.offline = Some(offline);
        self
    }

    /// In rare cases initGeetest can be called with a new_captcha parameter
    pub fn new_captcha(mut self, new_captcha: bool) -> Self {
        self.new_captcha = Some(new_captcha);
        self
    }

    /// Callback URL where you wish to receive the response
    pub fn pingback(mut self, pingback: impl Into<String>) -> Self {
        self.pingback = Some(pingback.into());
        self
    }

    /// The URL to your proxy server
    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }

    /// Your userAgent that will be used to solve the captcha
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
}
