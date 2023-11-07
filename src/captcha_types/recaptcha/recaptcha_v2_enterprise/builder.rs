use std::borrow::Cow;

use itertools::Itertools;

use crate::{
    prelude::*,
    proxy::Proxy,
    type_state::{UrlMissing, UrlProvided, WebsiteKeyMissing, WebsiteKeyProvided},
};

use super::task::{RecaptchaV2Enterprise, RecaptchaV2EnterpriseTypes};

/// This struct allows you to create a [`RecaptchaV2Enterprise`] struct
/// while checking at compile time that all required fields were
/// provided
pub struct RecaptchaV2EnterpriseBuilder<'a, T, U, V>
where
    V: serde::Serialize,
{
    proxy: Option<Proxy<'a>>,
    website_url: T,
    website_key: U,
    enterprise_payload: Option<V>,
    is_invisible: bool,
    user_agent: Option<Cow<'a, str>>,
    cookies: &'a [(&'a str, &'a str)],
    api_domain: Option<Cow<'a, str>>,
}

impl<'a, T> RecaptchaV2EnterpriseBuilder<'a, UrlProvided, WebsiteKeyProvided<'a>, T>
where
    T: serde::Serialize,
{
    pub fn build(self) -> RecaptchaV2Enterprise<'a, T> {
        let cookies = self
            .cookies
            .iter()
            .map(|x| format!("{}={}", x.0, x.1))
            .join(";");

        RecaptchaV2Enterprise {
            task_type: match self.proxy {
                Some(proxy) => RecaptchaV2EnterpriseTypes::WithProxy(proxy),
                None => RecaptchaV2EnterpriseTypes::ProxyLess,
            },
            website_url: self.website_url.0,
            website_key: self.website_key.0,
            enterprise_payload: self.enterprise_payload,
            is_invisible: self.is_invisible,
            user_agent: self.user_agent,
            cookies: if !cookies.is_empty() {
                Some(cookies.into())
            } else {
                None
            },
            api_domain: self.api_domain,
        }
    }
}

impl<T> RecaptchaV2EnterpriseBuilder<'_, UrlMissing, WebsiteKeyMissing, T>
where
    T: serde::Serialize,
{
    pub const fn new() -> Self {
        RecaptchaV2EnterpriseBuilder {
            proxy: None,
            website_url: UrlMissing,
            website_key: WebsiteKeyMissing,
            enterprise_payload: None,
            is_invisible: false,
            user_agent: None,
            cookies: &[],
            api_domain: None,
        }
    }
}

impl<T> Default for RecaptchaV2EnterpriseBuilder<'_, UrlMissing, WebsiteKeyMissing, T>
where
    T: serde::Serialize,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, U, V> RecaptchaV2EnterpriseBuilder<'a, T, U, V>
where
    V: serde::Serialize,
{
    /// Proxy login information
    pub fn proxy(mut self, proxy: Option<Proxy<'a>>) -> Self {
        self.proxy = proxy;
        self
    }

    /// The full URL of target web page where the captcha is loaded
    ///
    /// # Errors
    /// This function will error if the provided url is invalid
    pub fn website_url(
        self,
        website_url: &str,
    ) -> Result<RecaptchaV2EnterpriseBuilder<'a, UrlProvided, U, V>> {
        Ok(RecaptchaV2EnterpriseBuilder {
            proxy: self.proxy,
            website_url: UrlProvided(url::Url::parse(website_url)?),
            website_key: self.website_key,
            enterprise_payload: self.enterprise_payload,
            is_invisible: self.is_invisible,
            user_agent: self.user_agent,
            cookies: self.cookies,
            api_domain: self.api_domain,
        })
    }

    /// Can be found inside hte data-sitekey property of the reCAPTCHA
    /// `div` element or inside the `k` parameter of the requests to the
    /// reCAPTHCHA API.
    pub fn website_key(
        self,
        website_key: impl Into<Cow<'a, str>>,
    ) -> RecaptchaV2EnterpriseBuilder<'a, T, WebsiteKeyProvided<'a>, V> {
        RecaptchaV2EnterpriseBuilder {
            proxy: self.proxy,
            website_url: self.website_url,
            website_key: WebsiteKeyProvided(website_key.into()),
            enterprise_payload: self.enterprise_payload,
            is_invisible: self.is_invisible,
            user_agent: self.user_agent,
            cookies: self.cookies,
            api_domain: self.api_domain,
        }
    }

    /// Additional parameters passed to the `grecaptcha.enterprise.render` call.
    /// For example, there may be an object containing an `s` value
    pub fn enterprise_payload(mut self, enterprise_payload: Option<V>) -> Self {
        self.enterprise_payload = enterprise_payload;
        self
    }

    /// Pass `true` for the invisible version of reCAPTCHA - a case
    /// when you don't see the checkbox, but the challenge appears.
    /// Mostly used with a callback function
    pub fn invisible(mut self, is_invisible: bool) -> Self {
        self.is_invisible = is_invisible;
        self
    }

    /// User-Agent used to load the captcha.
    /// Use only modern browsers' User-Agents
    pub fn user_agent(mut self, user_agent: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.user_agent = user_agent.map(Into::into);
        self
    }

    /// Your cookies will be set in a browser of the worker.
    /// Suitable for captcha on Google services.
    pub fn cookies(mut self, cookies: &'a [(&'a str, &'a str)]) -> Self {
        self.cookies = cookies;
        self
    }

    /// Domain used to load the captcha: `google.com` or `recaptcha.net`.
    /// Default value: `google.com`
    pub fn api_domain(mut self, api_domain: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.api_domain = api_domain.map(Into::into);
        self
    }
}
