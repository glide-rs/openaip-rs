use Airspace;
use Error;

#[derive(Default, Debug)]
pub struct File {
    pub airspaces: Option<Vec<Result<Airspace, Error>>>,
}
