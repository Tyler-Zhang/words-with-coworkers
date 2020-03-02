use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    BadAction(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BadAction(ref err) => write!(f, "Bad Action error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        Some(self)
    }
}

pub type Result<T> = std::result::Result<T, Box<Error>>;
