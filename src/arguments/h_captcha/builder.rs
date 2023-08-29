use crate::arguments::{
    proxy::Proxy,
    type_state::{
        page_url::{PageUrl, PageUrlNotProvided},
        site_key::{SiteKey, SiteKeyNotProvided},
    },
    HCaptcha,
};

#[derive(Debug, Default, Clone)]
/// Builds a [`HCaptcha`] instance using the typestate pattern
/// to help avoid sending avoid inconsistent data to the
/// 2captcha API
///
/// # Example
/// ```
/// use captcha_oxide::arguments::HCaptcha;
///
/// let args = HCaptcha::builder()
///     .site_key("SITE_KEY")
///     .page_url("SOME_URL")
///     .build();
/// ```
pub struct HCaptchaBuilder<U, K> {
    page_url: U,
    site_key: K,
    invisible: Option<bool>,
    domain: Option<String>,
    data: Option<String>,
    pingback: Option<String>,
    proxy: Option<Proxy>,
}

impl HCaptchaBuilder<PageUrlNotProvided, SiteKeyNotProvided> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl HCaptchaBuilder<PageUrl, SiteKey> {
    pub fn build(self) -> HCaptcha {
        HCaptcha {
            site_key: self.site_key.0,
            page_url: self.page_url.0,
            invisible: self.invisible,
            domain: self.domain,
            data: self.data,
            pingback: self.pingback,
            proxy: self.proxy,
        }
    }
}

impl<U, K> HCaptchaBuilder<U, K> {
    /// Full URL of the page where you see the captcha
    pub fn page_url(self, page_url: impl Into<String>) -> HCaptchaBuilder<PageUrl, K> {
        HCaptchaBuilder {
            page_url: PageUrl(page_url.into()),
            site_key: self.site_key,
            invisible: self.invisible,
            domain: self.domain,
            data: self.data,
            pingback: self.pingback,
            proxy: self.proxy,
        }
    }

    /// Value of the data-sitekey attribute found in the page's HTML
    pub fn site_key(self, site_key: impl Into<String>) -> HCaptchaBuilder<U, SiteKey> {
        HCaptchaBuilder {
            page_url: self.page_url,
            site_key: SiteKey(site_key.into()),
            invisible: self.invisible,
            domain: self.domain,
            data: self.data,
            pingback: self.pingback,
            proxy: self.proxy,
        }
    }

    /// Whether or not the captcha you are dealing with the invisible
    /// version of hCaptcha. This is pretty rare as of mid 2023
    pub fn invisible(mut self, invisible: bool) -> Self {
        self.invisible = Some(invisible);
        self
    }

    /// Domain used to load the captcha, e.g.: hcaptcha.com or js.hcaptcha.com
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    /// Custom data that is used in some implementations of hCaptcha,
    /// mostly with invisible hCaptcha. In most cases you see it as rqdata
    /// inside the page's network requests.
    pub fn data(self, data: impl Into<String>) -> HCaptchaBuilder<U, K> {
        HCaptchaBuilder {
            page_url: self.page_url,
            site_key: self.site_key,
            invisible: self.invisible,
            domain: self.domain,
            data: Some(data.into()),
            pingback: self.pingback,
            proxy: self.proxy,
        }
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
}
