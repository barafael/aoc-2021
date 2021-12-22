use std::str::FromStr;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct Player {
    pub(crate) position: u16,
    pub(crate) score: u16,
}

impl Player {
    pub const fn new(position: u16) -> Self {
        Self { position, score: 0 }
    }
}

impl FromStr for Player {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, position) = input.split_once(": ").unwrap();
        let position = position.parse().unwrap();
        Ok(Self::new(position))
    }
}
