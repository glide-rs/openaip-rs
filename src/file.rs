use crate::Airspace;
use crate::Error;

#[derive(Default, Debug)]
pub struct File {
    pub airspaces: Option<Vec<Result<Airspace, Error>>>,
}
