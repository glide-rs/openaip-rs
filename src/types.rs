use std::fmt;
use std::str::FromStr;

use xmltree::Element;

use try_from::TryFrom;
use xml::ElementExt;

#[derive(Default, Debug)]
pub struct File {
    pub airspaces: Option<Vec<Airspace>>,
}

#[derive(Debug)]
pub struct Airspace {
    pub category: Category,
    pub version: String,
    pub id: String,
    pub country: String,
    pub name: String,
    pub top: AltitudeLimit,
    pub bottom: AltitudeLimit,
    pub geometry: Geometry,
}

impl<'a> TryFrom<&'a Element> for Airspace {
    type Err = ();

    fn try_from(element: &Element) -> Result<Self, Self::Err> {
        Ok(Airspace {
            category: element.get_attr("CATEGORY").unwrap().parse().unwrap(),
            version: element.get_child("VERSION").unwrap().text.as_ref().unwrap().clone(),
            id: element.get_child("ID").unwrap().text.as_ref().unwrap().clone(),
            country: element.get_child("COUNTRY").unwrap().text.as_ref().unwrap().clone(),
            name: element.get_child("NAME").unwrap().text.as_ref().unwrap().clone(),
            top: element.get_child("ALTLIMIT_TOP").unwrap().into(),
            bottom: element.get_child("ALTLIMIT_BOTTOM").unwrap().into(),
            geometry: element.get_child("GEOMETRY").unwrap().into(),
        })
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Category {
    A,
    B,
    C,
    CTR,
    D,
    Danger,
    E,
    F,
    FIR,
    G,
    Gliding,
    Other,
    Prohibited,
    Restricted,
    RMZ,
    TMA,
    TMZ,
    UIR,
    Wave,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "A" => Ok(Category::A),
            "B" => Ok(Category::B),
            "C" => Ok(Category::C),
            "CTR" => Ok(Category::CTR),
            "D" => Ok(Category::D),
            "DANGER" => Ok(Category::Danger),
            "E" => Ok(Category::E),
            "F" => Ok(Category::F),
            "FIR" => Ok(Category::FIR),
            "G" => Ok(Category::G),
            "GLIDING" => Ok(Category::Gliding),
            "OTH" => Ok(Category::Other),
            "PROHIBITED" => Ok(Category::Prohibited),
            "RESTRICTED" => Ok(Category::Restricted),
            "RMZ" => Ok(Category::RMZ),
            "TMA" => Ok(Category::TMA),
            "TMZ" => Ok(Category::TMZ),
            "UIR" => Ok(Category::UIR),
            "WAVE" => Ok(Category::Wave),
            _ => Err(()),
        }
    }
}

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
    /// # use openaip_parser::types::{AltitudeLimit, AltitudeReference, AltitudeUnit};
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

#[derive(Debug)]
pub enum Geometry {
    Polygon(Vec<Point>),
}

impl<'a> From<&'a Element> for Geometry {
    fn from(element: &Element) -> Self {
        let polygon = element.get_child("POLYGON").unwrap();
        let text = polygon.text.as_ref().unwrap();
        let points = text.split(",").map(|s| s.parse().unwrap());

        Geometry::Polygon(points.collect())
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub longitude: f64,
    pub latitude: f64,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        Ok(Point {
            longitude: parts[0].parse().unwrap(),
            latitude: parts[1].parse().unwrap(),
        })
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.longitude, self.latitude)
    }
}
