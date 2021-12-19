#[cfg(test)]
mod tests {
    use crate::day17::{probe::Probe, test::INPUT, TargetArea};
    use std::str::FromStr;

    #[test]
    fn brute_forces_result() {
        let target_area = TargetArea::from_str(INPUT).unwrap();
        let mut hit_counter = 0;
        for dx in -200..=200 {
            for dy in -200..=200 {
                let mut probe = Probe::new(dx, dy);

                loop {
                    if probe.overshot(&target_area) {
                        break;
                    }
                    if probe.is_in_target_area(&target_area) {
                        hit_counter += 1;
                        break;
                    }
                    probe.step()
                }
            }
        }
        assert_eq!(4716, hit_counter);
    }
}
