mod airspace;
mod altitude_limit;
mod altitude_reference;
mod altitude_unit;
mod category;
mod error;
mod file;
mod geometry;
mod point;
mod xml;

use std::convert::TryFrom;
use std::io::Read;

use xmltree::Element;

pub use crate::airspace::Airspace;
pub use crate::altitude_limit::AltitudeLimit;
pub use crate::altitude_reference::AltitudeReference;
pub use crate::altitude_unit::AltitudeUnit;
pub use crate::category::Category;
pub use crate::error::Error;
pub use crate::file::File;
use crate::file::File as OpenAipFile;
pub use crate::geometry::Geometry;
pub use crate::point::Point;
use crate::xml::ElementExt;

/// Parses an XML document into an `OpenAipFile` instance.
///
/// # Examples
///
/// ```
/// # use openaip::parse;
/// #
/// let data: &'static str = r##"
/// <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
/// <OPENAIP VERSION="d9192d6fa44fc5a0ecc3d84fd84d13e091df511c" DATAFORMAT="1.1">
///   <AIRSPACES>
///     <ASP CATEGORY="WAVE">
///       <VERSION>d59ffb1bd865bc7307dbb3a191f4d00dfef5529f</VERSION>
///       <ID>150668</ID>
///       <COUNTRY>DE</COUNTRY>
///       <NAME>Alb-Nord_1 128.950</NAME>
///       <ALTLIMIT_TOP REFERENCE="STD">
///         <ALT UNIT="FL">100</ALT>
///       </ALTLIMIT_TOP>
///       <ALTLIMIT_BOTTOM REFERENCE="MSL">
///         <ALT UNIT="F">4500</ALT>
///       </ALTLIMIT_BOTTOM>
///       <GEOMETRY>
///         <POLYGON>9.1911 48.4911, 9.3727 48.5655, 9.4222 48.5747, 9.1911 48.4911</POLYGON>
///       </GEOMETRY>
///     </ASP>
///   </AIRSPACES>
/// </OPENAIP>
/// "##;
///
/// let result = parse(data.as_bytes());
/// assert!(result.is_ok());
/// ```
pub fn parse<R: Read>(r: R) -> Result<OpenAipFile, Error> {
    let dom = Element::parse(r)?;
    if dom.name != "OPENAIP" {
        return Err(Error::MissingElement("OPENAIP"));
    }

    let data_format_version = dom.get_attr("DATAFORMAT")?;
    if data_format_version != "1.1" {
        return Err(Error::IncompatibleDataFormatVersion(
            data_format_version.clone(),
        ));
    }

    let file = OpenAipFile {
        airspaces: dom
            .get_child("AIRSPACES")
            .map(|e: &Element| e.children.iter().map(Airspace::try_from).collect()),
    };

    Ok(file)
}
