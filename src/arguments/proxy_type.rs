use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProxyType {
    HTTP,
    HTTPS,
    SOCKS4,
    SOCKS5,
}

impl ToString for ProxyType {
    fn to_string(&self) -> String {
        match self {
            ProxyType::HTTP => "HTTP",
            ProxyType::HTTPS => "HTTPS",
            ProxyType::SOCKS4 => "SOCKS4",
            ProxyType::SOCKS5 => "SOCKS5",
        }
        .into()
    }
}
