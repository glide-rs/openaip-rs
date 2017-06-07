extern crate xmltree;

mod airspace;
mod altitude_limit;
mod altitude_reference;
mod altitude_unit;
mod category;
mod file;
mod geometry;
mod point;
mod error;
mod try_from;
mod xml;

use std::io::Read;

use xmltree::Element;

pub use error::Error;
pub use airspace::Airspace;
pub use category::Category;
pub use file::File;
pub use geometry::Geometry;
pub use point::Point;
pub use altitude_unit::AltitudeUnit;
pub use altitude_reference::AltitudeReference;
pub use altitude_limit::AltitudeLimit;
use file::{File as OpenAipFile};
use try_from::TryFrom;
use xml::ElementExt;

pub fn parse<R: Read>(r: R) -> Result<OpenAipFile, Error> {
    let dom = Element::parse(r)?;
    if dom.name != "OPENAIP" {
        return Err(Error::MissingElement("OPENAIP"));
    }

    let data_format_version = dom.get_attr("DATAFORMAT")?;
    if data_format_version != "1.1" {
        return Err(Error::IncompatibleDataFormatVersion(data_format_version.clone()));
    }

    let file = OpenAipFile {
        airspaces: dom.get_child("AIRSPACES").map(|e: &Element| {
            e.children.iter().map(Airspace::try_from).collect()
        }),
    };

    Ok(file)
}
