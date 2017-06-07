use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub enum AltitudeUnit { Feet, FlightLevel }

impl FromStr for AltitudeUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "F" => Ok(AltitudeUnit::Feet),
            "FL" => Ok(AltitudeUnit::FlightLevel),
            _ => Err(()),
        }
    }
}
