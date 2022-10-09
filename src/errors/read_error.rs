use core::fmt;
use std::error::Error;
#[derive(Debug, Clone)]
pub struct ReadError;

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stream Read Error")
    }
}
impl Error for ReadError {}
