use itertools::Itertools;
use std::num::ParseIntError;
use thiserror::Error;

use super::fish::{self, Fish};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse number")]
    ParseInt(#[from] ParseIntError),

    #[error("Failed to create fish with error {0}")]
    CreateFish(#[from] fish::Error),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct World {
    day: usize,
    fish: Vec<Fish>,
}

impl World {
    #[must_use = "Don't let the fish go to waste!"]
    pub fn day(&mut self) -> Vec<Fish> {
        let mut new_fish = Vec::with_capacity(self.fish.len());
        for fish in &mut self.fish {
            if let Some(new) = fish.day() {
                new_fish.push(new);
            }
        }
        self.day += 1;
        new_fish
    }

    pub fn add_fish(&mut self, mut new_fish: Vec<Fish>) {
        self.fish.append(&mut new_fish);
    }

    pub fn how_many_fish(&self) -> usize {
        self.fish.len()
    }
}

impl From<Vec<Fish>> for World {
    fn from(fish: Vec<Fish>) -> Self {
        Self { day: 0, fish }
    }
}

impl TryFrom<&str> for World {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let fish = value
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<u8>, ParseIntError>>()?;
        let fish = fish
            .iter()
            .copied()
            .map(Fish::try_from)
            .collect::<Result<Vec<Fish>, fish::Error>>()?;
        Ok(Self::from(fish))
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.day {
            0 => write!(f, "Initial state: "),
            1 => write!(f, "After  1 day:  "),
            n if n < 10 => write!(f, "After  {} days: ", n),
            n => write!(f, "After {} days: ", n),
        }?;
        write!(f, "{}", self.fish.iter().format(","))?;
        writeln!(f)
    }
}

#[cfg(test)]
mod test {
    use super::World;
    use crate::day06::{naive::fish::Fish, test::INPUT};

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn parses_example_world() {
        let input = "3,4,3,1,2";
        let world = World::try_from(input).unwrap();
        assert_eq!(
            World {
                day: 0,
                fish: vec![
                    Fish::try_from(3).unwrap(),
                    Fish::try_from(4).unwrap(),
                    Fish::try_from(3).unwrap(),
                    Fish::try_from(1).unwrap(),
                    Fish::try_from(2).unwrap()
                ]
            },
            world
        );
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn parses_input_ok() {
        let world = World::try_from(INPUT);
        assert!(world.is_ok()); // I have hope.
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn formats_example_initial_world() {
        let fish: Vec<Fish> = vec![
            Fish::try_from(3).unwrap(),
            Fish::try_from(4).unwrap(),
            Fish::try_from(3).unwrap(),
            Fish::try_from(1).unwrap(),
            Fish::try_from(2).unwrap(),
        ];
        let world = World::from(fish);
        assert_eq!("Initial state: 3,4,3,1,2\n", format!("{}", world));
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn formats_example_young_world() {
        let fish = vec![
            Fish::try_from(2).unwrap(),
            Fish::try_from(3).unwrap(),
            Fish::try_from(2).unwrap(),
            Fish::try_from(0).unwrap(),
            Fish::try_from(1).unwrap(),
        ];
        let world = World { day: 1, fish };
        assert_eq!("After  1 day:  2,3,2,0,1\n", format!("{}", world));
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn formats_example_less_young_world() {
        let fish = vec![
            Fish::try_from(2).unwrap(),
            Fish::try_from(3).unwrap(),
            Fish::try_from(2).unwrap(),
            Fish::try_from(0).unwrap(),
            Fish::try_from(1).unwrap(),
            Fish::try_from(2).unwrap(),
            Fish::try_from(3).unwrap(),
            Fish::try_from(4).unwrap(),
            Fish::try_from(4).unwrap(),
            Fish::try_from(5).unwrap(),
        ];
        let world = World { day: 8, fish };
        assert_eq!("After  8 days: 2,3,2,0,1,2,3,4,4,5\n", format!("{}", world));
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn formats_example_late_world() {
        let fish = vec![
            Fish::try_from(6).unwrap(),
            Fish::try_from(0).unwrap(),
            Fish::try_from(6).unwrap(),
            Fish::try_from(4).unwrap(),
            Fish::try_from(5).unwrap(),
            Fish::try_from(6).unwrap(),
            Fish::try_from(0).unwrap(),
            Fish::try_from(1).unwrap(),
            Fish::try_from(1).unwrap(),
            Fish::try_from(2).unwrap(),
            Fish::try_from(6).unwrap(),
            Fish::try_from(7).unwrap(),
            Fish::try_from(8).unwrap(),
            Fish::try_from(8).unwrap(),
            Fish::try_from(8).unwrap(),
        ];
        let world = World { fish, day: 11 };
        assert_eq!(
            "After 11 days: 6,0,6,4,5,6,0,1,1,2,6,7,8,8,8\n",
            format!("{}", world)
        );
    }
}
