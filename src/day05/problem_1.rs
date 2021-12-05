use super::{map::Map, parse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse input")]
    Parse(#[from] parse::Error),
}

impl Map {
    pub fn count_overlaps(&mut self) -> Result<usize, Error> {
        Ok(self
            .entries
            .iter()
            .flat_map(|row| row.iter())
            .filter(|n| **n > 1)
            .count())
    }
}

#[cfg(test)]
mod tests {
    use crate::day05::{
        map::Map,
        test::{EXAMPLE_INPUT, INPUT},
        Sequence,
    };

    #[test]
    fn computes_basic_example() {
        let seq = Sequence::try_from(EXAMPLE_INPUT).unwrap();
        let seq = seq
            .0
            .iter()
            .filter(|n| n.start.x == n.end.x || n.start.y == n.end.y)
            .collect::<Vec<_>>();
        let mut map: Map = Map::new(10, 10);
        for line in seq {
            map.feed(line);
        }
        assert_eq!(5, map.count_overlaps().unwrap());
    }

    #[test]
    fn computes_result() {
        let seq = Sequence::try_from(INPUT).unwrap();
        let seq = seq
            .0
            .iter()
            .filter(|n| n.start.x == n.end.x || n.start.y == n.end.y)
            .collect::<Vec<_>>();
        let mut map: Map = Map::new(1000, 1000);
        for line in seq {
            map.feed(line);
        }
        assert_eq!(5690, map.count_overlaps().unwrap());
    }
}
