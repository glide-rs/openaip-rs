extern crate xmltree;

pub mod types;
mod error;
mod try_from;
mod xml;

use std::io::Read;

use xmltree::Element;

pub use error::Error;
use try_from::TryFrom;
use types::{File as OpenAipFile};
use types::*;
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
        airspaces: dom.get_child("AIRSPACES").map(convert_airspaces),
    };

    Ok(file)
}

fn convert_airspaces(airspaces: &Element) -> Vec<Airspace> {
    airspaces.children.iter().map(|e| Airspace::try_from(e).unwrap()).collect()
}
