use crate::proxy::Proxy;

mod challenge_page;
mod solution;
mod standalone;

pub use challenge_page::TurnstileChallengePageCaptcha;
pub use standalone::TurnstileStandaloneCaptcha;

#[derive(serde::Serialize)]
#[serde(tag = "type")]
#[catptcha_oxide_derive::from_option]
pub enum TurnstileCaptchaTypes<'a> {
    #[serde(rename = "TurnstileTaskProxyless")]
    ProxyLess,

    #[serde(rename = "TurnstileTask")]
    WithProxy(Proxy<'a>),
}
