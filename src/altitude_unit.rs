use std::str::FromStr;

use crate::Error;

/// `Feet` or `FlightLevel`
#[derive(Eq, PartialEq, Debug)]
pub enum AltitudeUnit {
    Feet,
    FlightLevel,
}

impl FromStr for AltitudeUnit {
    type Err = Error;

    /// # Examples
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use openaip::AltitudeUnit;
    /// #
    /// assert_eq!(AltitudeUnit::from_str("F").unwrap(), AltitudeUnit::Feet);
    /// assert_eq!(AltitudeUnit::from_str("FL").unwrap(), AltitudeUnit::FlightLevel);
    /// assert!(AltitudeUnit::from_str("foobar").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "F" => Ok(AltitudeUnit::Feet),
            "FL" => Ok(AltitudeUnit::FlightLevel),
            _ => Err(Error::UnknownAltitudeUnit(s.to_string())),
        }
    }
}
