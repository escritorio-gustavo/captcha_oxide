use crate::arguments::{
    proxy::Proxy,
    recaptcha_v2::RecaptchaV2,
    type_state::{
        page_url::{PageUrl, PageUrlNotProvided},
        site_key::{SiteKey, SiteKeyNotProvided},
    },
};

#[derive(Default, Debug, Clone)]
/// Builds a [`RecaptchaV2`] instance using the typestate pattern
/// to help avoid sending avoid inconsistent data to the
/// 2captcha API
///
/// # Example
/// ```
/// use captcha_oxide::arguments::RecaptchaV2;
///
/// let args = RecaptchaV2::builder()
///     .site_key("SOME_SITE_KEY")
///     .page_url("SOME_URL")
///     .build();
/// ```
pub struct RecaptchaV2Builder<T, U> {
    page_url: T,
    site_key: U,
    domain: Option<String>,
    data_s: Option<String>,
    user_agent: Option<String>,
    pingback: Option<String>,
    proxy: Option<Proxy>,
    enterprise: Option<bool>,
    invisible: Option<bool>,
}

impl RecaptchaV2Builder<PageUrlNotProvided, SiteKeyNotProvided> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RecaptchaV2Builder<PageUrl, SiteKey> {
    pub fn build(self) -> RecaptchaV2 {
        RecaptchaV2 {
            page_url: self.page_url.0,
            site_key: self.site_key.0,
            domain: self.domain,
            data_s: self.data_s,
            user_agent: self.user_agent,
            pingback: self.pingback,
            proxy: self.proxy,
            enterprise: self.enterprise,
            invisible: self.invisible,
        }
    }
}

impl<T, U> RecaptchaV2Builder<T, U> {
    /// Full URL of the page where you see the captcha
    pub fn page_url(self, page_url: impl Into<String>) -> RecaptchaV2Builder<PageUrl, U> {
        RecaptchaV2Builder {
            page_url: PageUrl(page_url.into()),
            site_key: self.site_key,
            domain: self.domain,
            data_s: self.data_s,
            user_agent: self.user_agent,
            pingback: self.pingback,
            proxy: self.proxy,
            enterprise: self.enterprise,
            invisible: self.invisible,
        }
    }

    /// Value of the sitekey parameter you found on the page
    pub fn site_key(self, site_key: impl Into<String>) -> RecaptchaV2Builder<T, SiteKey> {
        RecaptchaV2Builder {
            page_url: self.page_url,
            site_key: SiteKey(site_key.into()),
            domain: self.domain,
            data_s: self.data_s,
            user_agent: self.user_agent,
            pingback: self.pingback,
            proxy: self.proxy,
            enterprise: self.enterprise,
            invisible: self.invisible,
        }
    }

    /// Domain used to load the captcha, e.g.: google.com or recaptcha.net
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    /// Value of the data-s parameter you found on the page.
    /// Curenttly applicable for Google Search and other Google services.
    pub fn data_s(mut self, data_s: impl Into<String>) -> Self {
        self.data_s = Some(data_s.into());
        self
    }

    /// Your userAgent that will be used to solve the captcha
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
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

    /// Whether or not the page uses Enterprise reCAPTCHA
    pub fn enterprise(mut self, enterprise: bool) -> Self {
        self.enterprise = Some(enterprise);
        self
    }

    /// Whether or not the page uses Invisible reCAPTCHA
    pub fn invisible(mut self, invisible: bool) -> Self {
        self.invisible = Some(invisible);
        self
    }
}
