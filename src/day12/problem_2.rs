#[cfg(test)]
mod tests {
    use crate::day12::{get_paths, test::INPUT, Caves};
    use std::str::FromStr;

    #[test]
    fn computes_solution() {
        let caves = Caves::from_str(INPUT).unwrap();
        assert_eq!(107395, get_paths(&caves, true));
    }
}
