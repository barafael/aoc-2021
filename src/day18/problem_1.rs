#[cfg(test)]
mod tests {
    use crate::day18::{snailfish_math::parser::parse_input, test::INPUT};

    #[ignore]
    #[test]
    fn computes_solution() {
        let mut pairs = parse_input(INPUT);
        let initial = pairs.remove(0);
        let res = pairs.into_iter().fold(initial, |acc, cur| acc + cur);

        assert_eq!(3699, res.magnitude());
    }
}
