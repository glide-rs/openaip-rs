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
            version: element.get_element("VERSION")?.get_text()?.clone(),
            id: element.get_element("ID")?.get_text()?.clone(),
            country: element.get_element("COUNTRY")?.get_text()?.clone(),
            name: element.get_element("NAME")?.get_text()?.clone(),
            top: element.get_element("ALTLIMIT_TOP")?.into(),
            bottom: element.get_element("ALTLIMIT_BOTTOM")?.into(),
            geometry: Geometry::try_from(element.get_element("GEOMETRY")?)?,
        })
    }
}
