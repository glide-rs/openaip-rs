use std::str::FromStr;

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
    type Err = ();

    /// # Examples
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use openaip_parser::AltitudeReference;
    /// #
    /// assert_eq!(AltitudeReference::from_str("GND").unwrap(), AltitudeReference::GND);
    /// assert_eq!(AltitudeReference::from_str("STD").unwrap(), AltitudeReference::STD);
    /// assert_eq!(AltitudeReference::from_str("MSL").unwrap(), AltitudeReference::MSL);
    /// ```
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "GND" => Ok(AltitudeReference::GND),
            "STD" => Ok(AltitudeReference::STD),
            "MSL" => Ok(AltitudeReference::MSL),
            _ => Err(()),
        }
    }
}
