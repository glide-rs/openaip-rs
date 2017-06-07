use xmltree;

#[derive(Debug)]
pub enum Error {
    Xml(xmltree::ParseError),
}
