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

use minidom::{Element, NSChoice};

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

/// The XML namespace used by openAIP exports.
pub(crate) const OPENAIP_NAMESPACE: &str = "https://www.openaip.net";

/// Parses an XML document into an `OpenAipFile` instance.
///
/// # Examples
///
/// ```
/// # use openaip::parse;
/// #
/// let data: &'static str = r##"
/// <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
/// <OPENAIP xmlns="https://www.openaip.net" VERSION="d9192d6fa44fc5a0ecc3d84fd84d13e091df511c" DATAFORMAT="1.1">
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
/// let result = parse(data);
/// assert!(result.is_ok());
/// ```
pub fn parse(str: &str) -> Result<OpenAipFile, Error> {
    // The XML declaration must be the first thing in the document; the parser
    // rejects any leading whitespace before it, so trim it first.
    let dom = str.trim_start().parse::<Element>()?;
    if dom.name() != "OPENAIP" {
        return Err(Error::MissingElement("OPENAIP"));
    }

    let data_format_version = dom.get_attr("DATAFORMAT")?;
    if data_format_version != "1.1" {
        return Err(Error::IncompatibleDataFormatVersion(
            data_format_version.to_string(),
        ));
    }

    let file = OpenAipFile {
        airspaces: dom
            .get_child("AIRSPACES", NSChoice::OneOf(OPENAIP_NAMESPACE))
            .map(|e| e.children().map(Airspace::try_from).collect()),
    };

    Ok(file)
}
