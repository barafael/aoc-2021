#[cfg(test)]
mod tests {
    use crate::day05::{map::Map, test::INPUT, Sequence};

    #[cfg(feature = "non_solution_test")]
    const MAP_WITH_DIAGONALS: &str = r##"1.1....11.
.111...2..
..2.1.111.
...1.2.2..
.112313211
...1.2....
..1...1...
.1.....1..
1.......1.
222111....
"##;

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn computes_example() {
        let mut map = Map::new(10, 10);
        let seq = Sequence::try_from(crate::day05::test::EXAMPLE_INPUT).unwrap();
        for line in seq.0 {
            map.feed(&line);
        }
        assert_eq!(MAP_WITH_DIAGONALS, format!("{}", map));
    }

    #[test]
    fn computes_result() {
        let mut map = Map::new(1000, 1000);
        let seq = Sequence::try_from(INPUT).unwrap();
        for line in seq.0 {
            map.feed(&line);
        }
        assert_eq!(17741, map.count_overlaps().unwrap());
    }
}
