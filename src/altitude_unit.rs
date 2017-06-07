use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub enum AltitudeUnit { Feet, FlightLevel }

impl FromStr for AltitudeUnit {
    type Err = ();

    /// # Examples
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use openaip_parser::AltitudeUnit;
    /// #
    /// assert_eq!(AltitudeUnit::from_str("F").unwrap(), AltitudeUnit::Feet);
    /// assert_eq!(AltitudeUnit::from_str("FL").unwrap(), AltitudeUnit::FlightLevel);
    /// assert!(AltitudeUnit::from_str("foobar").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "F" => Ok(AltitudeUnit::Feet),
            "FL" => Ok(AltitudeUnit::FlightLevel),
            _ => Err(()),
        }
    }
}
