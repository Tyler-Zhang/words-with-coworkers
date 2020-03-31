use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    BadAction(String),
    NotEnoughTiles,
    InvalidWord(String),
    StartingTileNotCovered,
    WordDoesNotIntersect,
    NoLettersUsed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BadAction(ref err) => write!(f, "Bad Action error: {}", err),
            Error::NotEnoughTiles => write!(f, "Not enough tiles"),
            Error::InvalidWord(ref word) => write!(f, "Word <{}> not in the dictionary", word),
            Error::StartingTileNotCovered => write!(f, "Starting tile needs to be covered"),
            Error::WordDoesNotIntersect => write!(f, "Word does not intersect with another word"),
            Error::NoLettersUsed => write!(f, "You must use at least one letter"),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        Some(self)
    }
}

pub type Result<T> = std::result::Result<T, Box<Error>>;
