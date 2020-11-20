use minidom::{Element, NSChoice};

use crate::error::Error;

pub trait ElementExt {
    fn get_attr(&self, k: &'static str) -> Result<&str, Error>;
    fn get_element(&self, k: &'static str) -> Result<&Element, Error>;
}

impl ElementExt for Element {
    fn get_attr(&self, k: &'static str) -> Result<&str, Error> {
        self.attr(k).ok_or_else(|| Error::MissingAttribute(k))
    }

    fn get_element(&self, k: &'static str) -> Result<&Element, Error> {
        self.get_child(k, NSChoice::None)
            .ok_or_else(|| Error::MissingElement(k))
    }
}
