
use std::fmt::{Display, Formatter};
use std::io::{Error as IoError, ErrorKind};


#[derive(Debug)]
pub enum CyonixError {
     CustomError(ErrorKind),
     SpecificError(String)
}

//To provide a user-friendly error message
impl Display for CyonixError {
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
        CyonixError::CustomError(ErrorKind::PermissionDenied) => {
            write!(f, "Oops, permission is denied :(")
        }
        CyonixError::CustomError(ErrorKind::NotFound) => {
            write!(f, "Oops, the file/directory is not found:(")
        }
        CyonixError::CustomError(ErrorKind::AlreadyExists) => {
            write!(f, "Oops, the file/directory already exists :(")
        }
        CyonixError::SpecificError(err) => write!(f, "Oops: {}", err),
        _ => write!(f, "Oops, something went wrong :("),
    }
}
}
//To allow for easy conversion from std::io::Error to our custom error type
impl From<IoError> for CyonixError {
    fn from(err: IoError) -> CyonixError {
        CyonixError::CustomError(err.kind())
    }
}

