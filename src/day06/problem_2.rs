use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse number")]
    ParseInt(#[from] ParseIntError),

    #[error("Invalid fish age {0}")]
    InvalidFishAge(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct World {
    day: usize,
    fish_tank: [u64; 9],
}

impl TryFrom<&str> for World {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let fish = value
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<u8>, ParseIntError>>()?;
        Self::try_from(fish)
    }
}

impl World {
    pub fn day(&mut self) {
        let zero = self.fish_tank[0];
        for i in 0..8 {
            self.fish_tank[i] = self.fish_tank[i + 1];
        }
        self.fish_tank[6] += zero;
        self.fish_tank[8] = zero;
        self.day += 1;
    }

    pub fn how_many_fish(&self) -> u64 {
        self.fish_tank.iter().sum()
    }
}

impl TryFrom<Vec<u8>> for World {
    type Error = Error;

    fn try_from(values: Vec<u8>) -> Result<Self, Self::Error> {
        for value in &values {
            if value > &8 {
                return Err(Error::InvalidFishAge(*value));
            }
        }
        let mut world = Self::default();
        for value in values {
            world.fish_tank[value as usize] += 1;
        }
        Ok(world)
    }
}

#[cfg(test)]
mod tests {
    use crate::day06::{
        naive::{fish::Fish, world::World as NaiveWorld},
        problem_2::World,
        test::INPUT,
    };

    #[cfg(feature = "non_solution_test")]
    #[test]
    #[ignore = "naive solution has inadequately long run time :)"]
    fn simulates_example_problem_fish_count_naive() {
        let initial_fish = vec![
            Fish::try_from(3).unwrap(),
            Fish::try_from(4).unwrap(),
            Fish::try_from(3).unwrap(),
            Fish::try_from(1).unwrap(),
            Fish::try_from(2).unwrap(),
        ];
        let mut world: NaiveWorld = initial_fish.try_into().unwrap();
        for _ in 0..256 {
            let new_fish = world.day();
            world.add_fish(new_fish);
        }
        assert_eq!(26984457539, world.how_many_fish());
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn makes_fish_tank() {
        let world = World::try_from(vec![3, 4, 3, 1, 2]).unwrap();
        assert_eq!(
            World {
                day: 0,
                fish_tank: [0, 1, 1, 2, 1, 0, 0, 0, 0],
            },
            world
        );
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn simulates_example_problem_fish_count() {
        let initial_fish = vec![3, 4, 3, 1, 2];
        let mut world: World = initial_fish.try_into().unwrap();
        for _ in 0..18 {
            world.day();
        }
        assert_eq!(26, world.how_many_fish());
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn simulates_example_problem_fish_count_after_80_days() {
        let initial_fish = vec![3, 4, 3, 1, 2];
        let mut world: World = initial_fish.try_into().unwrap();
        for _ in 0..80 {
            world.day();
        }
        assert_eq!(5934, world.how_many_fish());
    }

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn simulates_example_problem_fish_count_after_256_days() {
        let initial_fish = vec![3, 4, 3, 1, 2];
        let mut world: World = initial_fish.try_into().unwrap();
        for _ in 0..256 {
            world.day();
        }
        assert_eq!(26984457539, world.how_many_fish());
    }

    #[test]
    fn simulates_fish_count_after_256_days() {
        let mut world = World::try_from(INPUT).unwrap();
        for _ in 0..256 {
            world.day();
        }
        assert_eq!(1675781200288, world.how_many_fish());
    }
}
