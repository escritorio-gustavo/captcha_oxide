use crate::arguments::{
    proxy::Proxy,
    type_state::{
        page_url::{PageUrl, PageUrlNotProvided},
        site_key::{SiteKey, SiteKeyNotProvided},
    },
    GeetestV4,
};

#[derive(Debug, Default, Clone)]
/// Builds a [`GeetestV4`] instance using the typestate pattern
/// to help avoid sending avoid inconsistent data to the
/// 2captcha API
///
/// # Example
/// ```
/// use captcha_oxide::arguments::GeetestV4;
///
/// let args = GeetestV4::builder()
///     .site_key("SITE_KEY")
///     .page_url("SOME_URL")
///     .build();
/// ```
pub struct GeetestV4Builder<T, U> {
    /// Value of the captcha_id parameter found in the page's HTML
    site_key: T,

    /// Full URL of the page where you see the captcha
    page_url: U,

    /// Callback URL where you wish to receive the response
    pingback: Option<String>,

    /// The URL to your proxy server
    proxy: Option<Proxy>,
}

impl GeetestV4Builder<SiteKeyNotProvided, PageUrlNotProvided> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl GeetestV4Builder<SiteKey, PageUrl> {
    pub fn build(self) -> GeetestV4 {
        GeetestV4 {
            site_key: self.site_key.0,
            page_url: self.page_url.0,
            pingback: self.pingback,
            proxy: self.proxy,
        }
    }
}

impl<T, U> GeetestV4Builder<T, U> {
    /// Value of the captcha_id parameter found in the page's HTML
    pub fn site_key(self, site_key: impl Into<String>) -> GeetestV4Builder<SiteKey, U> {
        GeetestV4Builder {
            site_key: SiteKey(site_key.into()),
            page_url: self.page_url,
            pingback: self.pingback,
            proxy: self.proxy,
        }
    }

    /// Full URL of the page where you see the captcha
    pub fn page_url(self, page_url: impl Into<String>) -> GeetestV4Builder<T, PageUrl> {
        GeetestV4Builder {
            site_key: self.site_key,
            page_url: PageUrl(page_url.into()),
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
