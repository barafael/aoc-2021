#[cfg(test)]
mod tests {
    use crate::day21::{situation::Situation, test::INPUT};
    use std::{cmp, collections::HashMap, str::FromStr};

    #[test]
    fn computes_solution() {
        let winning_universes = Situation::from_str(INPUT)
            .unwrap()
            .get_wins(&mut HashMap::new())
            .into_iter()
            .fold(0, |a, b| cmp::max(a, b));
        assert_eq!(91559198282731, winning_universes)
    }
}
