use xmltree::Element;

use xml::ElementExt;
use AltitudeLimit;
use Category;
use Geometry;
use try_from::TryFrom;
use Error;

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
    type Err = Error;

    fn try_from(element: &Element) -> Result<Self, Self::Err> {
        Ok(Airspace {
            category: element.get_attr("CATEGORY")?.parse()?,
            version: element.get_child("VERSION").unwrap().text.as_ref().unwrap().clone(),
            id: element.get_child("ID").unwrap().text.as_ref().unwrap().clone(),
            country: element.get_child("COUNTRY").unwrap().text.as_ref().unwrap().clone(),
            name: element.get_child("NAME").unwrap().text.as_ref().unwrap().clone(),
            top: element.get_child("ALTLIMIT_TOP").unwrap().into(),
            bottom: element.get_child("ALTLIMIT_BOTTOM").unwrap().into(),
            geometry: Geometry::try_from(element.get_child("GEOMETRY").unwrap())?,
        })
    }
}
