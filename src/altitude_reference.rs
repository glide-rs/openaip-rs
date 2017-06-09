use std::str::FromStr;

use Error;

#[derive(Eq, PartialEq, Debug)]
pub enum AltitudeReference {
    /// Ground
    GND,
    /// Mean sea level
    MSL,
    /// Standard atmosphere
    STD,
}

impl FromStr for AltitudeReference {
    type Err = Error;

    /// # Examples
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use openaip::AltitudeReference;
    /// #
    /// assert_eq!(AltitudeReference::from_str("GND").unwrap(), AltitudeReference::GND);
    /// assert_eq!(AltitudeReference::from_str("STD").unwrap(), AltitudeReference::STD);
    /// assert_eq!(AltitudeReference::from_str("MSL").unwrap(), AltitudeReference::MSL);
    /// assert!(AltitudeReference::from_str("foobar").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GND" => Ok(AltitudeReference::GND),
            "STD" => Ok(AltitudeReference::STD),
            "MSL" => Ok(AltitudeReference::MSL),
            _ => Err(Error::UnknownAltitudeReference(s.to_string())),
        }
    }
}
