mod geetest_v3;
mod geetest_v4;
mod type_state;

pub use geetest_v3::GeeTestV3;
pub use geetest_v4::GeeTestV4;

use crate::proxy::Proxy;

#[derive(serde::Serialize)]
#[serde(tag = "type")]
#[catptcha_oxide_derive::from_option]
pub enum GeetestTypes<'a> {
    #[serde(rename = "GeeTestTaskProxyless")]
    ProxyLess,

    #[serde(rename = "GeeTestTask")]
    WithProxy(Proxy<'a>),
}
