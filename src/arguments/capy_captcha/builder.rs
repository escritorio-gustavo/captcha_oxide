pub use super::type_state::{Version, VersionNotProvided};

use crate::arguments::{
    capy_captcha::{CapyCaptcha, CapyVersion},
    proxy::Proxy,
    type_state::{
        page_url::{PageUrl, PageUrlNotProvided},
        site_key::{SiteKey, SiteKeyNotProvided},
    },
};

#[derive(Default, Debug, Clone)]
/// Builds a [`CapyCaptcha`] instance using the typestate pattern
/// to help avoid sending avoid inconsistent data to the 2captcha
/// API
///
/// # Example
/// ```
/// use captcha_oxide::arguments::{
///     CaptchaArguments,
///     CapyCaptcha,
///     capy_captcha::CapyVersion
/// };
///
/// let capy_args = CapyCaptcha::builder()
///     .site_key("PUZZLE_Cme4hZLjuZRMYC3uh14C52D3uNms5w")
///     .page_url("https://www.capy.me/account/signin")
///     .version(CapyVersion::Puzzle)
///     .build();
/// ```
pub struct CapyCaptchaBuilder<U, K, V> {
    page_url: U,
    site_key: K,
    version: V,
    api_server: Option<String>,
    pingback: Option<String>,
    proxy: Option<Proxy>,
}

impl CapyCaptchaBuilder<PageUrlNotProvided, SiteKeyNotProvided, VersionNotProvided> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CapyCaptchaBuilder<PageUrl, SiteKey, Version> {
    pub fn build(self) -> CapyCaptcha {
        CapyCaptcha {
            site_key: self.site_key.0,
            page_url: self.page_url.0,
            version: self.version.0,
            api_server: self.api_server,
            pingback: self.pingback,
            proxy: self.proxy,
        }
    }
}

impl<U, K, V> CapyCaptchaBuilder<U, K, V> {
    /// The website's captcha key. You should be able to find this
    /// informatino in the site's HTML.
    pub fn page_url(self, page_url: impl Into<String>) -> CapyCaptchaBuilder<PageUrl, K, V> {
        CapyCaptchaBuilder {
            page_url: PageUrl(page_url.into()),
            site_key: self.site_key,
            version: self.version,
            api_server: self.api_server,
            pingback: self.pingback,
            proxy: self.proxy,
        }
    }

    /// Full URL of the page where you see the captcha
    pub fn site_key(self, site_key: impl Into<String>) -> CapyCaptchaBuilder<U, SiteKey, V> {
        CapyCaptchaBuilder {
            page_url: self.page_url,
            site_key: SiteKey(site_key.into()),
            version: self.version,
            api_server: self.api_server,
            pingback: self.pingback,
            proxy: self.proxy,
        }
    }

    /// The kind of puzzle to be solved
    pub fn version(self, version: CapyVersion) -> CapyCaptchaBuilder<U, K, Version> {
        CapyCaptchaBuilder {
            page_url: self.page_url,
            site_key: self.site_key,
            version: Version(version),
            api_server: self.api_server,
            pingback: self.pingback,
            proxy: self.proxy,
        }
    }

    /// The domain of the script's source URL
    pub fn api_server(mut self, api_server: impl Into<String>) -> Self {
        self.api_server = Some(api_server.into());
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
}
