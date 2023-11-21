use std::borrow::Cow;

use crate::{
    prelude::*,
    proxy::Proxy,
    type_state::{UrlMissing, UrlProvided, WebsiteKeyMissing, WebsiteKeyProvided},
};

use super::{
    type_state::{ContextMissing, ContextProvided, IvMissing, IvProvided},
    AmazonCaptcha,
};

pub struct AmazonCaptchaBuilder<'a, T, U, V, W> {
    website_url: T,
    website_key: U,
    iv: V,
    context: W,
    challenge_script: Option<Cow<'a, str>>,
    captcha_script: Option<Cow<'a, str>>,
    proxy: Option<Proxy<'a>>,
}

impl<'a>
    AmazonCaptchaBuilder<
        'a,
        UrlProvided<'a>,
        WebsiteKeyProvided<'a>,
        IvProvided<'a>,
        ContextProvided<'a>,
    >
{
    pub fn build(self) -> Result<AmazonCaptcha<'a>> {
        Ok(AmazonCaptcha {
            website_url: url::Url::parse(self.website_url.0)?,
            website_key: self.website_key.0,
            iv: self.iv.0,
            context: self.context.0,
            challenge_script: self.challenge_script,
            captcha_script: self.captcha_script,
            task_type: self.proxy.into(),
        })
    }
}

impl<'a> AmazonCaptchaBuilder<'a, UrlMissing, WebsiteKeyMissing, IvMissing, ContextMissing> {
    pub const fn new() -> Self {
        Self {
            website_url: UrlMissing,
            website_key: WebsiteKeyMissing,
            iv: IvMissing,
            context: ContextMissing,
            challenge_script: None,
            captcha_script: None,
            proxy: None,
        }
    }
}

impl<'a> Default
    for AmazonCaptchaBuilder<'a, UrlMissing, WebsiteKeyMissing, IvMissing, ContextMissing>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, U, V, W> AmazonCaptchaBuilder<'a, T, U, V, W> {
    pub fn website_url(
        self,
        website_url: &'a str,
    ) -> AmazonCaptchaBuilder<'a, UrlProvided<'a>, U, V, W> {
        AmazonCaptchaBuilder {
            website_url: UrlProvided(website_url),
            website_key: self.website_key,
            iv: self.iv,
            context: self.context,
            challenge_script: self.challenge_script,
            captcha_script: self.captcha_script,
            proxy: self.proxy,
        }
    }

    pub fn website_key(
        self,
        website_key: impl Into<Cow<'a, str>>,
    ) -> AmazonCaptchaBuilder<'a, T, WebsiteKeyProvided<'a>, V, W> {
        AmazonCaptchaBuilder {
            website_url: self.website_url,
            website_key: WebsiteKeyProvided(website_key.into()),
            iv: self.iv,
            context: self.context,
            challenge_script: self.challenge_script,
            captcha_script: self.captcha_script,
            proxy: self.proxy,
        }
    }

    pub fn iv(
        self,
        iv: impl Into<Cow<'a, str>>,
    ) -> AmazonCaptchaBuilder<'a, T, U, IvProvided<'a>, W> {
        AmazonCaptchaBuilder {
            website_url: self.website_url,
            website_key: self.website_key,
            iv: IvProvided(iv.into()),
            context: self.context,
            challenge_script: self.challenge_script,
            captcha_script: self.captcha_script,
            proxy: self.proxy,
        }
    }

    pub fn context(
        self,
        context: impl Into<Cow<'a, str>>,
    ) -> AmazonCaptchaBuilder<'a, T, U, V, ContextProvided<'a>> {
        AmazonCaptchaBuilder {
            website_url: self.website_url,
            website_key: self.website_key,
            iv: self.iv,
            context: ContextProvided(context.into()),
            challenge_script: self.challenge_script,
            captcha_script: self.captcha_script,
            proxy: self.proxy,
        }
    }

    pub fn challenge_script(mut self, challenge_script: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.challenge_script = challenge_script.map(Into::into);
        self
    }

    pub fn captcha_script(mut self, captcha_script: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.captcha_script = captcha_script.map(Into::into);
        self
    }

    pub fn proxy(mut self, proxy: Option<Proxy<'a>>) -> Self {
        self.proxy = proxy;
        self
    }
}
