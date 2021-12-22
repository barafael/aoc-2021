#[cfg(test)]
mod tests {
    use crate::day20::{test::INPUT, trenchmap::TrenchMap};
    use std::str::FromStr;

    #[test]
    fn computes_solution() {
        let map = TrenchMap::from_str(INPUT).unwrap();
        let mut image = map.image.clone();
        image.apply_enhancement(&map.enhancement);
        image.apply_enhancement(&map.enhancement);
        assert_eq!(5432, image.count_light());
    }
}
