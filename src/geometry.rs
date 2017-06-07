use xmltree::Element;

use Point;

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
