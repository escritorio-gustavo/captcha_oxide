use std::borrow::Cow;

use itertools::Itertools;

use crate::{
    captcha_types::recaptcha::type_state::{NoUrlProvided, NoWebsiteKeyProvided, Url, WebsiteKey},
    proxy::Proxy,
};

use super::task::{RecaptchaV2, RecaptchaV2Types};

/// This struct allows you to create a [`RecaptchaV2`] struct
/// while checking at compile time that all required fields were
/// provided
pub struct RecaptchaV2Builder<'a, T, U> {
    proxy: Option<Proxy<'a>>,
    website_url: T,
    website_key: U,
    recaptcha_data_s_value: Option<Cow<'a, str>>,
    is_invisible: bool,
    user_agent: Option<Cow<'a, str>>,
    cookies: &'a [(&'a str, &'a str)],
    api_domain: Option<Cow<'a, str>>,
}

impl<'a> RecaptchaV2Builder<'a, Url, WebsiteKey<'a>> {
    pub fn build(self) -> RecaptchaV2<'a> {
        let cookies = self
            .cookies
            .iter()
            .map(|x| format!("{}={}", x.0, x.1))
            .join(";");
        RecaptchaV2 {
            task_type: match self.proxy {
                Some(proxy) => RecaptchaV2Types::WithProxy(proxy),
                None => RecaptchaV2Types::ProxyLess,
            },
            website_url: self.website_url.0,
            website_key: self.website_key.0,
            recaptcha_data_s_value: self.recaptcha_data_s_value,
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

impl RecaptchaV2Builder<'_, NoUrlProvided, NoWebsiteKeyProvided> {
    pub const fn new() -> Self {
        RecaptchaV2Builder {
            proxy: None,
            website_url: NoUrlProvided,
            website_key: NoWebsiteKeyProvided,
            recaptcha_data_s_value: None,
            is_invisible: false,
            user_agent: None,
            cookies: &[],
            api_domain: None,
        }
    }
}

impl Default for RecaptchaV2Builder<'_, NoUrlProvided, NoWebsiteKeyProvided> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, U> RecaptchaV2Builder<'a, T, U> {
    /// Proxy login information
    pub fn proxy(mut self, proxy: Option<Proxy<'a>>) -> Self {
        self.proxy = proxy;
        self
    }

    /// The full URL of target web page where the captcha is loaded
    pub fn website_url(self, website_url: url::Url) -> RecaptchaV2Builder<'a, Url, U> {
        RecaptchaV2Builder {
            proxy: self.proxy,
            website_url: Url(website_url),
            website_key: self.website_key,
            recaptcha_data_s_value: self.recaptcha_data_s_value,
            is_invisible: self.is_invisible,
            user_agent: self.user_agent,
            cookies: self.cookies,
            api_domain: self.api_domain,
        }
    }

    /// Can be found inside hte data-sitekey property of the reCAPTCHA
    /// `div` element or inside the `k` parameter of the requests to the
    /// reCAPTHCHA API.
    pub fn website_key(
        self,
        website_key: impl Into<Cow<'a, str>>,
    ) -> RecaptchaV2Builder<'a, T, WebsiteKey<'a>> {
        RecaptchaV2Builder {
            proxy: self.proxy,
            website_url: self.website_url,
            website_key: WebsiteKey(website_key.into()),
            recaptcha_data_s_value: self.recaptcha_data_s_value,
            is_invisible: self.is_invisible,
            user_agent: self.user_agent,
            cookies: self.cookies,
            api_domain: self.api_domain,
        }
    }

    /// The value of the `data-s` parameter. Can be required to bypass
    /// the captcha on Google services
    pub fn recaptcha_data_s_value(
        mut self,
        recaptcha_data_s_value: Option<impl Into<Cow<'a, str>>>,
    ) -> Self {
        self.recaptcha_data_s_value = recaptcha_data_s_value.map(Into::into);
        self
    }

    /// Pass `true` for the invisible version of reCAPTCHA - a case
    /// when you don't see the checkbox, but the challenge appears.
    /// Mostly used with a callback function
    pub fn is_invisible(mut self, is_invisible: bool) -> Self {
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
