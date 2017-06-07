extern crate xmltree;

pub mod types;
mod error;

use std::io::Read;

use xmltree::Element;

pub use error::Error;
use types::{File as OpenAipFile};
use types::*;

pub fn parse<R: Read>(r: R) -> Result<OpenAipFile, Error> {
    let dom = Element::parse(r).map_err(Error::Xml)?;
    if dom.name != "OPENAIP" {
        // error
    }

    if dom.attributes.get("DATAFORMAT").unwrap() != "1.1" {
        // error
    }

    let file = OpenAipFile {
        airspaces: dom.get_child("AIRSPACES").map(convert_airspaces),
    };

    Ok(file)
}

fn convert_airspaces(airspaces: &Element) -> Vec<Airspace> {
    airspaces.children.iter().map(|e| e.into()).collect()
}
