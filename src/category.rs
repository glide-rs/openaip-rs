use std::str::FromStr;

use error::Error;

#[derive(Eq, PartialEq, Debug)]
pub enum Category {
    A,
    B,
    C,
    CTR,
    D,
    Danger,
    E,
    F,
    FIR,
    G,
    Gliding,
    Other,
    Prohibited,
    Restricted,
    RMZ,
    TMA,
    TMZ,
    UIR,
    Wave,
}

impl FromStr for Category {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Category::A),
            "B" => Ok(Category::B),
            "C" => Ok(Category::C),
            "CTR" => Ok(Category::CTR),
            "D" => Ok(Category::D),
            "DANGER" => Ok(Category::Danger),
            "E" => Ok(Category::E),
            "F" => Ok(Category::F),
            "FIR" => Ok(Category::FIR),
            "G" => Ok(Category::G),
            "GLIDING" => Ok(Category::Gliding),
            "OTH" => Ok(Category::Other),
            "PROHIBITED" => Ok(Category::Prohibited),
            "RESTRICTED" => Ok(Category::Restricted),
            "RMZ" => Ok(Category::RMZ),
            "TMA" => Ok(Category::TMA),
            "TMZ" => Ok(Category::TMZ),
            "UIR" => Ok(Category::UIR),
            "WAVE" => Ok(Category::Wave),
            _ => Err(Error::UnknownCategory(s.to_string())),
        }
    }
}
