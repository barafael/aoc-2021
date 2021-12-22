use super::{image::Image, Enhancement};
use std::str::FromStr;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TrenchMap {
    pub(crate) enhancement: Enhancement,
    pub(crate) image: Image,
}

impl FromStr for TrenchMap {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (a, b) = input.split_once("\n\n").unwrap();
        Ok(Self {
            enhancement: Enhancement::from_str(a).unwrap(),
            image: Image::from_str(b).unwrap(),
        })
    }
}
