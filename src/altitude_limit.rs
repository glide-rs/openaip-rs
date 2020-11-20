use std::convert::TryFrom;
use std::fmt;

use minidom::Element;

use crate::xml::ElementExt;
use crate::AltitudeReference;
use crate::AltitudeUnit;
use crate::Error;

/// # Examples
///
/// ```
/// # use openaip::{AltitudeLimit, AltitudeReference, AltitudeUnit};
/// #
/// let limit1 = AltitudeLimit::new(4500, AltitudeUnit::Feet, AltitudeReference::MSL);
///
/// let limit2 = AltitudeLimit::new(65, AltitudeUnit::FlightLevel, AltitudeReference::STD);
/// ```
#[derive(Debug)]
pub struct AltitudeLimit {
    pub reference: AltitudeReference,
    pub unit: AltitudeUnit,
    pub value: i32,
}

impl AltitudeLimit {
    pub fn new(value: i32, unit: AltitudeUnit, reference: AltitudeReference) -> Self {
        AltitudeLimit {
            reference,
            unit,
            value,
        }
    }
}

impl<'a> TryFrom<&'a Element> for AltitudeLimit {
    type Error = Error;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let alt = element.get_element("ALT")?;

        let reference = element.get_attr("REFERENCE")?.parse()?;
        let unit = alt.get_attr("UNIT")?.parse()?;
        let value = alt.text().parse()?;

        Ok(AltitudeLimit {
            reference,
            unit,
            value,
        })
    }
}

impl fmt::Display for AltitudeLimit {
    /// # Examples
    ///
    /// ```
    /// # use openaip::{AltitudeLimit, AltitudeReference, AltitudeUnit};
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
