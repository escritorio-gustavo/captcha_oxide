use std::borrow::Cow;

use captcha_oxide_derive::proxy_task;
use url::Url;

use crate::CaptchaTask;

#[proxy_task(
    with_proxy = "AntiCyberSiAraTask",
    proxyless = "AntiCyberSiAraTaskProxyless"
)]
#[derive(CaptchaTask)]
#[task(timeout = 20)]
pub struct CyberSiARACaptcha<'a> {
    #[serde(rename = "websiteURL")]
    pub(super) website_url: Url,

    #[serde(rename = "SlideMasterUrlId")]
    pub(super) slide_master_url_id: Cow<'a, str>,
    pub(super) user_agent: Cow<'a, str>,
}
