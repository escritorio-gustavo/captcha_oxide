use std::borrow::Cow;

pub use self::{address::Address, proxy_type::ProxyType};

pub mod address;
pub mod proxy_type;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Proxy<'a> {
    pub proxy_type: ProxyType,
    pub proxy_address: Address,
    pub proxy_port: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_login: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_password: Option<Cow<'a, str>>,
}
