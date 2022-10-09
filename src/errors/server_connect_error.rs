use core::fmt;
use std::error::Error;
#[derive(Debug, Clone)]
pub struct ServerConnectError {
    pub ip: String,
}

impl fmt::Display for ServerConnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error connecting to {}", self.ip)
    }
}
impl Error for ServerConnectError {}
