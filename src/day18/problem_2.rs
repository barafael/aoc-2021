#[cfg(test)]
mod tests {
    use crate::day18::{snailfish_math::parser::parse_input, test::INPUT};

    #[test]
    fn computes_solution() {
        let pairs = parse_input(INPUT);

        let mut results = Vec::with_capacity(pairs.len() * pairs.len());

        for a in 0..pairs.len() {
            for b in 0..pairs.len() {
                if a == b {
                    continue;
                }

                results.push((pairs[a].clone() + pairs[b].clone()).magnitude());
            }
        }

        assert_eq!(4735, *results.iter().max().unwrap());
    }
}
