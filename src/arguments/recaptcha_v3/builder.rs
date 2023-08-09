use crate::arguments::{
    recaptcha_v3::RecaptchaV3,
    type_state::{
        page_url::{PageUrl, PageUrlNotProvided},
        site_key::{SiteKey, SiteKeyNotProvided},
    },
};

#[derive(Default, Debug, Clone)]
/// Builds a [`RecaptchaV3`] instance using the typestate pattern
/// to help avoid sending avoid inconsistent data to the
/// 2captcha API
///
/// # Example
/// ```
/// use captcha_oxide::arguments::RecaptchaV3;
///
/// let args = RecaptchaV3::builder()
///     .site_key("SOME_SITE_KEY")
///     .page_url("SOME_URL")
///     .min_score(0.3)
///     .build();
/// ```
pub struct RecaptchaV3Builder<T, U> {
    page_url: T,
    site_key: U,
    enterprise: Option<bool>,
    domain: Option<String>,
    action: Option<String>,
    min_score: Option<f32>,
    pingback: Option<String>,
}

impl RecaptchaV3Builder<PageUrlNotProvided, SiteKeyNotProvided> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RecaptchaV3Builder<PageUrl, SiteKey> {
    pub fn build(self) -> RecaptchaV3 {
        RecaptchaV3 {
            page_url: self.page_url.0,
            site_key: self.site_key.0,
            enterprise: self.enterprise,
            domain: self.domain,
            action: self.action,
            min_score: self.min_score,
            pingback: self.pingback,
        }
    }
}

impl<T, U> RecaptchaV3Builder<T, U> {
    /// Full URL of the page where you see the captcha
    pub fn page_url(self, page_url: impl Into<String>) -> RecaptchaV3Builder<PageUrl, U> {
        RecaptchaV3Builder {
            page_url: PageUrl(page_url.into()),
            site_key: self.site_key,
            enterprise: self.enterprise,
            domain: self.domain,
            action: self.action,
            min_score: self.min_score,
            pingback: self.pingback,
        }
    }

    /// Value of the sitekey parameter you found on the page
    pub fn site_key(self, site_key: impl Into<String>) -> RecaptchaV3Builder<T, SiteKey> {
        RecaptchaV3Builder {
            page_url: self.page_url,
            site_key: SiteKey(site_key.into()),
            enterprise: self.enterprise,
            domain: self.domain,
            action: self.action,
            min_score: self.min_score,
            pingback: self.pingback,
        }
    }

    /// Whether or not the page uses Enterprise reCAPTCHA
    pub fn enterprise(mut self, enterprise: bool) -> Self {
        self.enterprise = Some(enterprise);
        self
    }

    /// Domain used to load the captcha, e.g.: google.com or recaptcha.net
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    /// Value of the action parameter you found on the page
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// The score needed for resolution
    pub fn min_score(mut self, min_score: f32) -> Self {
        self.min_score = Some(min_score);
        self
    }

    /// Callback URL where you wish to receive the response
    pub fn pingback(mut self, pingback: impl Into<String>) -> Self {
        self.pingback = Some(pingback.into());
        self
    }
}
