use core::fmt;
use std::error::Error;
#[derive(Debug, Clone)]
pub struct EncryptionError;

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Encryption Error")
    }
}
impl Error for EncryptionError {}
