use std::error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidHangulError;

impl fmt::Display for InvalidHangulError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid hangul character")
    }
}

impl error::Error for InvalidHangulError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}