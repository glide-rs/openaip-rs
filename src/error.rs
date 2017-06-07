use std::error;
use std::fmt;

use xmltree;

#[derive(Debug)]
pub enum Error {
    Xml(xmltree::ParseError),
    MissingOpenAipElement,
    MissingAttribute(&'static str),
    IncompatibleDataFormatVersion(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Xml(ref err) => err.fmt(f),
            Error::MissingOpenAipElement => write!(f, "Missing <OPENAIP> element"),
            Error::MissingAttribute(ref name) => write!(f, "Missing {} attribute", name),
            Error::IncompatibleDataFormatVersion(ref version) => {
                write!(f, "Incompatible DATAFORMAT version: {}", version)
            },
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Xml(ref err) => err.description(),
            Error::MissingOpenAipElement => "Missing <OPENAIP> element",
            Error::MissingAttribute(..) => "Missing attribute",
            Error::IncompatibleDataFormatVersion(..) => "Incompatible DATAFORMAT version",
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
