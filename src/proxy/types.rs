use std::str::FromStr;
use crate::logging::{trace};

#[derive(Clone, Debug)]
pub enum ProxyProtocol {
    Http,
    Https,
    Socks4,
    Socks5,
}

#[derive(Clone, Debug)]
pub struct Proxy {
    pub protocol: ProxyProtocol,
    pub host: String,
    pub port: u16,
}

impl FromStr for Proxy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        trace(format!("converting string {} into protocol,host,port",s).as_str());
        let url = url::Url::parse(s)?;
        let protocol = match url.scheme() {
            "http" => ProxyProtocol::Http,
            "https" => ProxyProtocol::Https,
            "socks4" => ProxyProtocol::Socks4,
            "socks5" => ProxyProtocol::Socks5,
            _ => anyhow::bail!("Unsupported proxy protocol"),
        };
        trace(format!("protocol: {:?}",protocol).as_str());
        trace(format!("url: {:?}",url).as_str());
        trace(format!("port: {:?}",url.port_or_known_default()).as_str());


        Ok(Self {
            protocol,
            host: url.host_str().unwrap().to_string(),
            port: url.port_or_known_default().unwrap()
        })
    }
}
impl ToString for Proxy {
    fn to_string(&self) -> String {
        let scheme = match self.protocol {
            ProxyProtocol::Http => "http",
            ProxyProtocol::Https => "https",
            ProxyProtocol::Socks4 => "socks4",
            ProxyProtocol::Socks5 => "socks5",
        };
        format!("{}://{}:{}", scheme, self.host, self.port)
    }
}