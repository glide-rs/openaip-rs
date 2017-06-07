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

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "GND" => Ok(AltitudeReference::GND),
            "STD" => Ok(AltitudeReference::STD),
            "MSL" => Ok(AltitudeReference::MSL),
            _ => Err(()),
        }
    }
}
