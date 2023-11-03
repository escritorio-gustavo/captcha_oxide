use std::borrow::Cow;

use crate::captcha_types::type_state::{
    MinScore, NoMinScoreProvided, NoUrlProvided, NoWebsiteKeyProvided, Url, WebsiteKey,
};

use super::task::RecaptchaV3;

/// This struct allows you to create a [`RecaptchaV2`] struct
/// while checking at compile time that all required fields were
/// provided
pub struct RecaptchaV3Builder<'a, T, U, V> {
    website_url: T,
    website_key: U,
    min_score: V,
    page_action: Option<Cow<'a, str>>,
    is_enterprise: bool,
    api_domain: Option<Cow<'a, str>>,
}

impl<'a> RecaptchaV3Builder<'a, Url, WebsiteKey<'a>, MinScore> {
    pub fn build(self) -> RecaptchaV3<'a> {
        RecaptchaV3 {
            website_url: self.website_url.0,
            website_key: self.website_key.0,
            min_score: self.min_score.0,
            page_action: self.page_action,
            is_enterprise: self.is_enterprise,
            api_domain: self.api_domain,
        }
    }
}

impl<'a> RecaptchaV3Builder<'a, NoUrlProvided, NoWebsiteKeyProvided, NoMinScoreProvided> {
    pub const fn new() -> Self {
        Self {
            website_url: NoUrlProvided,
            website_key: NoWebsiteKeyProvided,
            min_score: NoMinScoreProvided,
            page_action: None,
            is_enterprise: false,
            api_domain: None,
        }
    }
}

impl<'a> Default
    for RecaptchaV3Builder<'a, NoUrlProvided, NoWebsiteKeyProvided, NoMinScoreProvided>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, U, V> RecaptchaV3Builder<'a, T, U, V> {
    /// The full URL of target web page where the captcha is loaded
    pub fn website_url(self, website_url: url::Url) -> RecaptchaV3Builder<'a, Url, U, V> {
        RecaptchaV3Builder {
            website_url: Url(website_url),
            website_key: self.website_key,
            min_score: self.min_score,
            page_action: self.page_action,
            is_enterprise: self.is_enterprise,
            api_domain: self.api_domain,
        }
    }

    /// Can be found inside hte data-sitekey property of the reCAPTCHA
    /// `div` element or inside the `k` parameter of the requests to the
    /// reCAPTHCHA API.
    pub fn website_key(
        self,
        website_key: impl Into<Cow<'a, str>>,
    ) -> RecaptchaV3Builder<'a, T, WebsiteKey<'a>, V> {
        RecaptchaV3Builder {
            website_url: self.website_url,
            website_key: WebsiteKey(website_key.into()),
            min_score: self.min_score,
            page_action: self.page_action,
            is_enterprise: self.is_enterprise,
            api_domain: self.api_domain,
        }
    }

    /// Required score value. The provided value will be clamped between 0 and 1.
    /// The 2captcha API docs recommend using either 0.3, 0.7 or 0.9
    pub fn min_score(self, min_score: f32) -> RecaptchaV3Builder<'a, T, U, MinScore> {
        RecaptchaV3Builder {
            website_url: self.website_url,
            website_key: self.website_key,
            min_score: MinScore(min_score.min(1.0f32).max(0.0f32)),
            page_action: self.page_action,
            is_enterprise: self.is_enterprise,
            api_domain: self.api_domain,
        }
    }

    /// Action parameter value. The value is set by the website owner inside
    /// the `data-action` attribute of the reCAPTCHA `div` element or passed
    /// inside the options object of the `execute` method call,
    /// e.g.: `grecaptcha.execute('websiteKey', { action: 'myAction' })`
    pub fn page_action(mut self, page_action: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.page_action = page_action.map(Into::into);
        self
    }

    /// Pass true for the Enterprise version of reCAPTCHA. You can identify
    /// it by the enterprise.js script used instead of api.js or by
    /// the `grecaptcha.enterprise.execute` call used instead of
    /// `grecaptcha.execute`
    pub fn enterprise(mut self, is_enterprise: bool) -> Self {
        self.is_enterprise = is_enterprise;
        self
    }

    /// Domain used to load the captcha: `google.com` or `recaptcha.net`.
    /// Default value: `google.com`
    pub fn api_domain(mut self, api_domain: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.api_domain = api_domain.map(Into::into);
        self
    }
}
