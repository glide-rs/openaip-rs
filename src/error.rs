use std::error;
use std::fmt;

use xmltree;

#[derive(Debug)]
pub enum Error {
    Xml(xmltree::ParseError),
    MissingElement(&'static str),
    MissingAttribute(&'static str),
    IncompatibleDataFormatVersion(String),
    UnknownCategory(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Xml(ref err) => err.fmt(f),
            Error::MissingElement(ref name) => write!(f, "Missing <{}> element", name),
            Error::MissingAttribute(ref name) => write!(f, "Missing {} attribute", name),
            Error::IncompatibleDataFormatVersion(ref version) => {
                write!(f, "Incompatible DATAFORMAT version: {}", version)
            },
            Error::UnknownCategory(ref s) => write!(f, "Unknown airspace category: {}", s),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Xml(ref err) => err.description(),
            Error::MissingElement(..) => "Missing element",
            Error::MissingAttribute(..) => "Missing attribute",
            Error::IncompatibleDataFormatVersion(..) => "Incompatible DATAFORMAT version",
            Error::UnknownCategory(..) => "Unknown airspace category",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Xml(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<xmltree::ParseError> for Error {
    fn from(err: xmltree::ParseError) -> Error {
        Error::Xml(err)
    }
}
