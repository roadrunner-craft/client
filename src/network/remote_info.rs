// TODO: remove this dead_code attribute when we have a menu for this
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct RemoteInfo {
    pub ip: String,
    pub port: u16,
}

impl RemoteInfo {
    #[allow(dead_code)]
    pub fn new(ip: String, port: u16) -> Self {
        Self { ip, port }
    }
}
