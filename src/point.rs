use std::fmt;
use std::str::FromStr;

use Error;

#[derive(Copy, Clone)]
pub struct Point {
    pub longitude: f64,
    pub latitude: f64,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(Error::InvalidPoint);
        }

        Ok(Point {
            longitude: parts[0].parse()?,
            latitude: parts[1].parse()?,
        })
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.longitude, self.latitude)
    }
}
