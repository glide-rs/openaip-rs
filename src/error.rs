use std::error;
use std::fmt;

use xmltree;

#[derive(Debug)]
pub enum Error {
    Xml(xmltree::ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Xml(ref err) => write!(f, "XML error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Xml(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Xml(ref err) => Some(err),
        }
    }
}
