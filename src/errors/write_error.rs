use core::fmt;
use std::error::Error;
#[derive(Debug, Clone)]
pub struct WriteError;

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stream Read Error")
    }
}
impl Error for WriteError {}
