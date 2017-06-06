use std::fmt;
use std::str::FromStr;

use xmltree::Element;

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

impl<'a> From<&'a Element> for Airspace {
    fn from(element: &Element) -> Self {
        Airspace {
            category: element.attributes.get("CATEGORY").unwrap().parse().unwrap(),
            version: element.get_child("VERSION").unwrap().clone().text.unwrap(),
            id: element.get_child("ID").unwrap().clone().text.unwrap(),
            country: element.get_child("COUNTRY").unwrap().clone().text.unwrap(),
            name: element.get_child("NAME").unwrap().clone().text.unwrap(),
            top: element.get_child("ALTLIMIT_TOP").unwrap().into(),
            bottom: element.get_child("ALTLIMIT_BOTTOM").unwrap().into(),
            geometry: element.get_child("GEOMETRY").unwrap().into(),
        }
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

impl<'a> From<&'a Element> for AltitudeLimit {
    fn from(element: &Element) -> Self {
        let alt = element.get_child("ALT").unwrap();

        AltitudeLimit {
            reference: element.attributes.get("REFERENCE").unwrap().parse().unwrap(),
            unit: alt.attributes.get("UNIT").unwrap().parse().unwrap(),
            value: alt.clone().text.unwrap().parse().unwrap(),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum AltitudeReference { GND, MSL, STD }

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
        let ref text = polygon.clone().text.unwrap();
        let points = text.split(",").map(|s| {
            let parts: Vec<&str> = s.split_whitespace().collect();
            Point {
                longitude: parts[0].parse().unwrap(),
                latitude: parts[1].parse().unwrap(),
            }
        });

        Geometry::Polygon(points.collect())
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub longitude: f64,
    pub latitude: f64,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.longitude, self.latitude)
    }
}
