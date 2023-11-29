use std::borrow::Cow;

use captcha_oxide_derive::{proxy_task, CaptchaTask};
use url::Url;

#[proxy_task(
    with_proxy = "AntiCyberSiAraTask",
    proxyless = "AntiCyberSiAraTaskProxyless"
)]
#[derive(CaptchaTask)]
#[task(timeout = 20)]
#[serde(rename_all = "camelCase")]
pub struct CutCaptcha<'a> {
    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,
    pub(super) misery_key: Cow<'a, str>,
    pub(super) api_key: Cow<'a, str>,
}
