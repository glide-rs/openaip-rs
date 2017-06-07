use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct Point {
    pub longitude: f64,
    pub latitude: f64,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        Ok(Point {
            longitude: parts[0].parse().unwrap(),
            latitude: parts[1].parse().unwrap(),
        })
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.longitude, self.latitude)
    }
}
