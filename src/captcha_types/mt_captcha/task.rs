use captcha_oxide_derive::{proxy_task, CaptchaTask};
use std::borrow::Cow;
use url::Url;

#[proxy_task(with_proxy = "MtCaptchaTask", proxyless = "MtCaptchaTaskProxyless")]
#[derive(CaptchaTask)]
#[task(timeout = 20)]
pub struct MtCaptcha<'a> {
    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) website_key: Cow<'a, str>,
}
