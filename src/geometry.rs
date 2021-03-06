use minidom::Element;
use std::convert::TryFrom;

use crate::xml::ElementExt;
use crate::Error;
use crate::Point;

#[derive(Debug)]
pub enum Geometry {
    Polygon(Vec<Point>),
}

impl<'a> TryFrom<&'a Element> for Geometry {
    type Error = Error;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let points: Result<Vec<_>, _> = element
            .get_element("POLYGON")?
            .text()
            .split(',')
            .map(|s| s.parse())
            .collect();

        Ok(Geometry::Polygon(points?))
    }
}
