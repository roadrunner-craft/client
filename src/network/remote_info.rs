#[derive(Debug, Clone, PartialEq)]
pub struct RemoteInfo {
    pub ip: String,
    pub port: u16,
}

impl RemoteInfo {
    pub fn new(ip: String, port: u16) -> Self {
        Self { ip, port }
    }
}
