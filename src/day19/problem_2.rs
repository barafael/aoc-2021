#[cfg(test)]
mod tests {
    use crate::day19::{test::INPUT, Scanners};
    use std::collections::HashSet;

    #[test]
    fn computes_solution() {
        let scanners = INPUT.parse::<Scanners>().unwrap();
        let rotated = scanners.rotate_all();
        let mut hash_set = HashSet::new();
        for r in &rotated.scanners {
            hash_set.extend(r.beacons.iter().copied());
        }
        assert_eq!(12292, rotated.max_manhattan_distance());
    }
}
