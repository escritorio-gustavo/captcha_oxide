use std::borrow::Cow;

use crate::{
    prelude::*,
    proxy::Proxy,
    type_state::{UrlMissing, UrlProvided, WebsiteKeyMissing, WebsiteKeyProvided},
};

use super::task::{HCaptcha, HCaptchaTypes};

/// This struct allows you to create a [`HCaptcha`] struct
/// while checking at compile time that all required fields were
/// provided
pub struct HCaptchaBuilder<'a, T, U, V>
where
    V: serde::Serialize,
{
    proxy: Option<Proxy<'a>>,
    website_url: T,
    website_key: U,
    is_invisible: bool,
    enterprise_payload: Option<V>,
}

impl<'a, T> HCaptchaBuilder<'a, UrlProvided, WebsiteKeyProvided<'a>, T>
where
    T: serde::Serialize,
{
    pub fn build(self) -> HCaptcha<'a, T> {
        HCaptcha {
            task_type: match self.proxy {
                Some(proxy) => HCaptchaTypes::WithProxy(proxy),
                None => HCaptchaTypes::ProxyLess,
            },
            website_url: self.website_url.0,
            website_key: self.website_key.0,
            is_invisible: self.is_invisible,
            enterprise_payload: self.enterprise_payload,
        }
    }
}

impl<T> HCaptchaBuilder<'_, UrlMissing, WebsiteKeyMissing, T>
where
    T: serde::Serialize,
{
    pub const fn new() -> Self {
        HCaptchaBuilder {
            proxy: None,
            website_url: UrlMissing,
            website_key: WebsiteKeyMissing,
            is_invisible: false,
            enterprise_payload: None,
        }
    }
}

impl<T> Default for HCaptchaBuilder<'_, UrlMissing, WebsiteKeyMissing, T>
where
    T: serde::Serialize,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, U, V> HCaptchaBuilder<'a, T, U, V>
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
    pub fn website_url(self, website_url: &str) -> Result<HCaptchaBuilder<'a, UrlProvided, U, V>> {
        Ok(HCaptchaBuilder {
            proxy: self.proxy,
            website_url: UrlProvided(url::Url::parse(website_url)?),
            website_key: self.website_key,
            is_invisible: self.is_invisible,
            enterprise_payload: self.enterprise_payload,
        })
    }

    /// Can be found inside hte data-sitekey property of the reCAPTCHA
    /// `div` element or inside the `k` parameter of the requests to the
    /// reCAPTHCHA API.
    pub fn website_key(
        self,
        website_key: impl Into<Cow<'a, str>>,
    ) -> HCaptchaBuilder<'a, T, WebsiteKeyProvided<'a>, V> {
        HCaptchaBuilder {
            proxy: self.proxy,
            website_url: self.website_url,
            website_key: WebsiteKeyProvided(website_key.into()),
            is_invisible: self.is_invisible,
            enterprise_payload: self.enterprise_payload,
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
}
