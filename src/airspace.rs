use xmltree::Element;

use try_from::TryFrom;
use xml::ElementExt;
use AltitudeLimit;
use Category;
use Error;
use Geometry;

#[derive(Debug)]
pub struct Airspace {
    pub category: Category,
    /// An openAIP specific value that represents the git commit ID that included this airspace
    pub version: String,
    /// An openAIP specific value that represents the internal ID of this airspace
    pub id: String,
    /// The airspace's ISO 3166-1 alpha-2 country code
    pub country: String,
    /// The airspace name
    pub name: String,
    /// The airspace upper ceiling
    pub top: AltitudeLimit,
    /// The airspace lower ceiling
    pub bottom: AltitudeLimit,
    /// The airspace geometry element
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
            top: AltitudeLimit::try_from(element.get_element("ALTLIMIT_TOP")?)?,
            bottom: AltitudeLimit::try_from(element.get_element("ALTLIMIT_BOTTOM")?)?,
            geometry: Geometry::try_from(element.get_element("GEOMETRY")?)?,
        })
    }
}
