use serde::{Deserialize, Serialize};

use super::proxy_type::ProxyType;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
/// This struct represents the credentials to your proxy server
pub struct Proxy {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,

    pub proxy_type: ProxyType,
}

impl ToString for Proxy {
    fn to_string(&self) -> String {
        format!(
            "{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}
