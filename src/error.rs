use std::fmt::{Display, Debug, Formatter};
use std::io::{Error, ErrorKind};
pub enum CyonixError {
     IoError(Error),
}

impl CyonixError {
    pub fn io_error(kind: ErrorKind, mes: &str) -> CyonixError {
        CyonixError::IoError(Error::new(kind, mes.to_string()))
    }
}

impl Display for CyonixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CyonixError::IoError(_error) => write!(f, "Failed to create a file :("),
        }
    }
}

impl Debug for CyonixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CyonixError::IoError(_error) => write!(f, "Failed to create a file :("),
        }
    }
}

