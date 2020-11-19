use xmltree::Element;

use try_from::TryFrom;
use xml::ElementExt;
use Error;
use Point;

#[derive(Debug)]
pub enum Geometry {
    Polygon(Vec<Point>),
}

impl<'a> TryFrom<&'a Element> for Geometry {
    type Err = Error;

    fn try_from(element: &Element) -> Result<Self, Self::Err> {
        let points: Result<Vec<_>, _> = element
            .get_element("POLYGON")?
            .get_text()?
            .split(',')
            .map(|s| s.parse())
            .collect();

        Ok(Geometry::Polygon(points?))
    }
}
