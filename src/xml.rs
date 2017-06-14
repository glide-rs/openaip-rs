use xmltree::Element;

use error::Error;

pub trait ElementExt {
    fn get_attr(&self, k: &'static str) -> Result<&String, Error>;
    fn get_element(&self, k: &'static str) -> Result<&Element, Error>;
    fn get_text(&self) -> Result<&String, Error>;
}

impl ElementExt for Element {
    fn get_attr(&self, k: &'static str) -> Result<&String, Error> {
        self.attributes.get(k).ok_or_else(|| Error::MissingAttribute(k))
    }
    fn get_element(&self, k: &'static str) -> Result<&Element, Error> {
        self.get_child(k).ok_or_else(|| Error::MissingElement(k))
    }
    fn get_text(&self) -> Result<&String, Error> {
        self.text.as_ref().ok_or(Error::MissingText)
    }
}

