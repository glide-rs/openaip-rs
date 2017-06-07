use std::error;
use std::fmt;
use std::num;

use xmltree;

#[derive(Debug)]
pub enum Error {
    Xml(xmltree::ParseError),
    InvalidNumber(num::ParseFloatError),
    MissingElement(&'static str),
    MissingAttribute(&'static str),
    IncompatibleDataFormatVersion(String),
    UnknownCategory(String),
    InvalidPoint,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Xml(ref err) => err.fmt(f),
            Error::InvalidNumber(ref err) => err.fmt(f),
            Error::MissingElement(ref name) => write!(f, "Missing <{}> element", name),
            Error::MissingAttribute(ref name) => write!(f, "Missing {} attribute", name),
            Error::IncompatibleDataFormatVersion(ref version) => {
                write!(f, "Incompatible DATAFORMAT version: {}", version)
            },
            Error::UnknownCategory(ref s) => write!(f, "Unknown airspace category: {}", s),
            Error::InvalidPoint => write!(f, "Invalid point"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Xml(ref err) => err.description(),
            Error::InvalidNumber(ref err) => err.description(),
            Error::MissingElement(..) => "Missing element",
            Error::MissingAttribute(..) => "Missing attribute",
            Error::IncompatibleDataFormatVersion(..) => "Incompatible DATAFORMAT version",
            Error::UnknownCategory(..) => "Unknown airspace category",
            Error::InvalidPoint => "Invalid point",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Xml(ref err) => Some(err),
            Error::InvalidNumber(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<xmltree::ParseError> for Error {
    fn from(err: xmltree::ParseError) -> Error {
        Error::Xml(err)
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(err: num::ParseFloatError) -> Error {
        Error::InvalidNumber(err)
    }
}
