#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Xml(#[from] minidom::Error),
    #[error(transparent)]
    InvalidIntNumber(#[from] std::num::ParseIntError),
    #[error(transparent)]
    InvalidFloatNumber(#[from] std::num::ParseFloatError),
    #[error("Missing <{0}> element")]
    MissingElement(&'static str),
    #[error("Missing <{0}> attribute")]
    MissingAttribute(&'static str),
    #[error("Missing element text")]
    MissingText,
    #[error("Incompatible DATAFORMAT version: {0}")]
    IncompatibleDataFormatVersion(String),
    #[error("Unknown airspace category: {0}")]
    UnknownCategory(String),
    #[error("Unknown altitude reference: {0}")]
    UnknownAltitudeReference(String),
    #[error("Unknown altitude unit: {0}")]
    UnknownAltitudeUnit(String),
    #[error("Invalid point")]
    InvalidPoint,
}
