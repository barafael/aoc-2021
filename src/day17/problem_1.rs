#[cfg(test)]
mod tests {
    use crate::day17::{test::INPUT, TargetArea, probe::Probe};
    use std::str::FromStr;

    #[test]
    fn example_trajectory() {
        let mut probe = Probe::new(7, 2);
        let target_area = TargetArea::from_str("target area: x=20..30, y=-10..-5").unwrap();
        probe.step();
        probe.step();
        probe.step();
        probe.step();
        probe.step();
        probe.step();
        probe.step();
        assert!(probe.is_in_target_area(&target_area));
    }

    #[test]
    fn brute_forces_result() {
        let target_area = TargetArea::from_str(INPUT).unwrap();
        let mut max_y = 0;
        for dx in -200..=200 {
            for dy in -200..=200 {
                let mut probe = Probe::new(dx, dy);
                let mut local_max_y = 0;

                loop {
                    if probe.overshot(&target_area) {
                        break;
                    }
                    if probe.is_in_target_area(&target_area) {
                        if local_max_y > max_y {
                            max_y = local_max_y;
                        }
                        break;
                    }
                    if probe.y > local_max_y {
                        local_max_y = probe.y
                    }
                    probe.step()
                }
            }
        }
        assert_eq!(10878, max_y);
    }
}
