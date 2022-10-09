use core::fmt;
use std::error::Error;
#[derive(Debug, Clone)]
pub struct InvalidServerAddressError {
    pub ip: String,
}

impl fmt::Display for InvalidServerAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Server IP: {}", self.ip)
    }
}
impl Error for InvalidServerAddressError {}
