use std::fmt;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum GetCommandStringError {
    IoError(std::io::Error),
    FromUtf8Error(FromUtf8Error),
}

// Implement the Display trait for the GetCommandStringError enum
impl fmt::Display for GetCommandStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetCommandStringError::IoError(msg) => write!(f, "{}", msg),
            GetCommandStringError::FromUtf8Error(code) => write!(f, "{}", code),
        }
    }
}

// Implement the From trait to convert FromUtf8Error into GetCommandStringError
impl From<FromUtf8Error> for GetCommandStringError {
    fn from(err: FromUtf8Error) -> GetCommandStringError {
        GetCommandStringError::FromUtf8Error(err)
    }
}

// Optionally, implement the From trait for std::io::Error as well
impl From<std::io::Error> for GetCommandStringError {
    fn from(err: std::io::Error) -> GetCommandStringError {
        GetCommandStringError::IoError(err)
    }
}
