use std::net::Ipv4Addr;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Address {
    IpAddress(Ipv4Addr),
    HostName(Box<str>),
}
