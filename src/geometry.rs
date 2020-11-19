use std::convert::TryFrom;
use xmltree::Element;

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
            .get_text()
            .ok_or(Error::MissingText)?
            .split(',')
            .map(|s| s.parse())
            .collect();

        Ok(Geometry::Polygon(points?))
    }
}
