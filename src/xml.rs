use xmltree::Element;

use error::Error;

pub trait ElementExt {
    fn get_attr(&self, k: &'static str) -> Result<&String, Error>;
}

impl ElementExt for Element {
    fn get_attr(&self, k: &'static str) -> Result<&String, Error> {
        self.attributes.get(k).ok_or(Error::MissingAttribute(k))
    }
}

