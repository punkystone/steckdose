use core::fmt;
use std::error::Error;
#[derive(Debug, Clone)]
pub struct JSONError;

impl fmt::Display for JSONError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Response Data")
    }
}
impl Error for JSONError {}
