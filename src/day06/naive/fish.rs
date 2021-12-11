use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid fish age {0}")]
    InvalidFishAge(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Fish(u8);

impl Default for Fish {
    fn default() -> Self {
        Self(8)
    }
}

impl std::fmt::Display for Fish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for Fish {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=8 => Ok(Self(value)),
            n => Err(Error::InvalidFishAge(n)),
        }
    }
}

impl Fish {
    pub const fn age(&self) -> u8 {
        self.0
    }

    #[must_use = "It would be disrespectful to drop a fish like that. Without caring if it even exists."]
    pub fn day(&mut self) -> Option<Self> {
        if self.0 == 0 {
            self.0 = 6;
            Some(Self::default())
        } else {
            self.0 -= 1;
            None
        }
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "non_solution_test")]
    use crate::day06::naive::fish::Fish;

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn fish_life_goals() {
        let mut fish = Fish::default();
        for age in (1..=8).rev() {
            assert_eq!(age, fish.age());
            assert!(fish.day().is_none());
        }
        assert_eq!(0, fish.age());
        let new_fish = fish.day().unwrap();
        assert_eq!(8, new_fish.age());
        assert_eq!(6, fish.age());
    }
}
