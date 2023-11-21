use std::borrow::Cow;

use crate::{
    prelude::*,
    proxy::Proxy,
    type_state::{UrlMissing, UrlProvided, WebsiteKeyMissing, WebsiteKeyProvided},
};

use super::{
    type_state::{
        ActionMissing, ActionProvided, DataMissing, DataProvided, PageDataMissing,
        PageDataProvided, UserAgentMissing, UserAgentProvided,
    },
    TurnstileChallengePageCaptcha,
};

pub struct TurnstileChallengePageCaptchaBuilder<'a, T, U, V, W, X, Y> {
    website_url: T,
    website_key: U,
    user_agent: V,
    action: W,
    data: X,
    page_data: Y,
    proxy: Option<Proxy<'a>>,
}

impl<'a>
    TurnstileChallengePageCaptchaBuilder<
        'a,
        UrlProvided<'a>,
        WebsiteKeyProvided<'a>,
        UserAgentProvided<'a>,
        ActionProvided<'a>,
        DataProvided<'a>,
        PageDataProvided<'a>,
    >
{
    pub fn build(self) -> Result<TurnstileChallengePageCaptcha<'a>> {
        Ok(TurnstileChallengePageCaptcha {
            task_type: self.proxy.into(),
            website_url: url::Url::parse(self.website_url.0)?,
            website_key: self.website_key.0,
            user_agent: self.user_agent.0,
            action: self.action.0,
            data: self.data.0,
            page_data: self.page_data.0,
        })
    }
}

impl<'a>
    TurnstileChallengePageCaptchaBuilder<
        'a,
        UrlMissing,
        WebsiteKeyMissing,
        UserAgentMissing,
        ActionMissing,
        DataMissing,
        PageDataMissing,
    >
{
    pub const fn new() -> Self {
        Self {
            website_url: UrlMissing,
            website_key: WebsiteKeyMissing,
            user_agent: UserAgentMissing,
            action: ActionMissing,
            data: DataMissing,
            page_data: PageDataMissing,
            proxy: None,
        }
    }
}

impl<'a> Default
    for TurnstileChallengePageCaptchaBuilder<
        'a,
        UrlMissing,
        WebsiteKeyMissing,
        UserAgentMissing,
        ActionMissing,
        DataMissing,
        PageDataMissing,
    >
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, U, V, W, X, Y> TurnstileChallengePageCaptchaBuilder<'a, T, U, V, W, X, Y> {
    pub fn website_url(
        self,
        website_url: &str,
    ) -> TurnstileChallengePageCaptchaBuilder<'a, UrlProvided, U, V, W, X, Y> {
        TurnstileChallengePageCaptchaBuilder {
            website_url: UrlProvided(website_url),
            website_key: self.website_key,
            user_agent: self.user_agent,
            action: self.action,
            data: self.data,
            page_data: self.page_data,
            proxy: self.proxy,
        }
    }

    pub fn website_key(
        self,
        website_key: impl Into<Cow<'a, str>>,
    ) -> TurnstileChallengePageCaptchaBuilder<'a, T, WebsiteKeyProvided<'a>, V, W, X, Y> {
        TurnstileChallengePageCaptchaBuilder {
            website_url: self.website_url,
            website_key: WebsiteKeyProvided(website_key.into()),
            user_agent: self.user_agent,
            action: self.action,
            data: self.data,
            page_data: self.page_data,
            proxy: self.proxy,
        }
    }

    pub fn user_agent(
        self,
        user_agent: impl Into<Cow<'a, str>>,
    ) -> TurnstileChallengePageCaptchaBuilder<'a, T, U, UserAgentProvided<'a>, W, X, Y> {
        TurnstileChallengePageCaptchaBuilder {
            website_url: self.website_url,
            website_key: self.website_key,
            user_agent: UserAgentProvided(user_agent.into()),
            action: self.action,
            data: self.data,
            page_data: self.page_data,
            proxy: self.proxy,
        }
    }

    pub fn action(
        self,
        action: impl Into<Cow<'a, str>>,
    ) -> TurnstileChallengePageCaptchaBuilder<'a, T, U, V, ActionProvided<'a>, X, Y> {
        TurnstileChallengePageCaptchaBuilder {
            website_url: self.website_url,
            website_key: self.website_key,
            user_agent: self.user_agent,
            action: ActionProvided(action.into()),
            data: self.data,
            page_data: self.page_data,
            proxy: self.proxy,
        }
    }

    pub fn data(
        self,
        data: impl Into<Cow<'a, str>>,
    ) -> TurnstileChallengePageCaptchaBuilder<'a, T, U, V, W, DataProvided<'a>, Y> {
        TurnstileChallengePageCaptchaBuilder {
            website_url: self.website_url,
            website_key: self.website_key,
            user_agent: self.user_agent,
            action: self.action,
            data: DataProvided(data.into()),
            page_data: self.page_data,
            proxy: self.proxy,
        }
    }

    pub fn page_data(
        self,
        page_data: impl Into<Cow<'a, str>>,
    ) -> TurnstileChallengePageCaptchaBuilder<'a, T, U, V, W, X, PageDataProvided<'a>> {
        TurnstileChallengePageCaptchaBuilder {
            website_url: self.website_url,
            website_key: self.website_key,
            user_agent: self.user_agent,
            action: self.action,
            data: self.data,
            page_data: PageDataProvided(page_data.into()),
            proxy: self.proxy,
        }
    }

    pub fn proxy(mut self, proxy: Option<Proxy<'a>>) -> Self {
        self.proxy = proxy;
        self
    }
}
