use std::fmt;

use xmltree::Element;

use xml::ElementExt;
use AltitudeUnit;
use AltitudeReference;

#[derive(Debug)]
pub struct AltitudeLimit {
    pub reference: AltitudeReference,
    pub unit: AltitudeUnit,
    pub value: i32,
}

impl AltitudeLimit {
    pub fn new(value: i32, unit: AltitudeUnit, reference: AltitudeReference) -> Self {
        AltitudeLimit { reference: reference, unit: unit, value: value }
    }
}

impl<'a> From<&'a Element> for AltitudeLimit {
    fn from(element: &Element) -> Self {
        let alt = element.get_child("ALT").unwrap();

        AltitudeLimit {
            reference: element.get_attr("REFERENCE").unwrap().parse().unwrap(),
            unit: alt.get_attr("UNIT").unwrap().parse().unwrap(),
            value: alt.text.as_ref().unwrap().parse().unwrap(),
        }
    }
}

impl fmt::Display for AltitudeLimit {
    /// # Examples
    ///
    /// ```
    /// # use openaip_parser::{AltitudeLimit, AltitudeReference, AltitudeUnit};
    /// #
    /// let limit = AltitudeLimit::new(4500, AltitudeUnit::Feet, AltitudeReference::MSL);
    /// assert_eq!(format!("{}", limit), "4500ft MSL");
    ///
    /// let limit = AltitudeLimit::new(65, AltitudeUnit::FlightLevel, AltitudeReference::STD);
    /// assert_eq!(format!("{}", limit), "FL65");
    ///
    /// let limit = AltitudeLimit::new(1500, AltitudeUnit::Feet, AltitudeReference::GND);
    /// assert_eq!(format!("{}", limit), "1500ft GND");
    ///
    /// let limit = AltitudeLimit::new(0, AltitudeUnit::Feet, AltitudeReference::GND);
    /// assert_eq!(format!("{}", limit), "GND");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.unit == AltitudeUnit::FlightLevel {
            write!(f, "FL{}", self.value)
        } else if self.reference == AltitudeReference::GND && self.value == 0 {
            write!(f, "GND")
        } else {
            let suffix = match self.reference {
                AltitudeReference::GND => "GND",
                AltitudeReference::MSL => "MSL",
                AltitudeReference::STD => "STD",
            };
            write!(f, "{}ft {}", self.value, suffix)
        }
    }
}
