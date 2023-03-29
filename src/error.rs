use std::fmt::{Display, Formatter};
use std::io::Error;


#[derive(Debug)]
pub enum CyonixError {
     IoError(Error),
     CustomError(String),
}

//To provide a user-friendly error message
impl Display for CyonixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CyonixError::IoError(e) => write!(f, "I/O error: {} :(", e),
            CyonixError::CustomError(err) => write!(f, "Error occurred: {} :(", err),
        }
    }
}

//To make it compatible with other error handling patterns
impl std::error::Error for CyonixError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CyonixError::IoError(e) => Some(e),
            CyonixError::CustomError(_) => None,
        }
    }
}

//To allow for easy conversion from std::io::Error to our custom error type
impl From<Error> for CyonixError {
    fn from(err: Error) -> CyonixError {
        CyonixError::IoError(err)
    }
}

