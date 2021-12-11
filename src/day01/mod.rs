use std::num::ParseIntError;

pub fn sonar_sweep(measurements: &[u32]) -> usize {
    measurements
        .windows(2)
        .filter(|win| win[0] < win[1])
        .count()
}

pub fn sonar_sweep_filtered(measurements: &[u32]) -> usize {
    let sums = measurements
        .windows(3)
        .map(|win| win.iter().sum())
        .collect::<Vec<u32>>();
    sonar_sweep(&sums)
}

pub fn buff_to_vec(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[cfg(test)]
mod tests {
    use super::{buff_to_vec, sonar_sweep, sonar_sweep_filtered};

    const INPUT: &str = include_str!("../../input/day01.txt");

    #[test]
    fn sonar_sweep_returns_2_increases() {
        let measurements = vec![199, 200, 208];
        assert_eq!(sonar_sweep(&measurements), 2);
    }

    #[test]
    fn sonar_sweep_returns_3_increases() {
        let measurements = vec![199, 200, 208, 210];
        assert_eq!(sonar_sweep(&measurements), 3);
    }

    #[test]
    fn sonar_sweep_returns_7_increases() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(sonar_sweep(&measurements), 7);
    }

    #[test]
    fn sonar_sweep_returns_0_increases_on_emtpy_array() {
        let measurements = vec![];
        assert_eq!(sonar_sweep(&measurements), 0);
    }

    #[test]
    fn sonar_sweep_returns_amount_of_increases() {
        let measurements = buff_to_vec(&INPUT).unwrap();
        assert_eq!(sonar_sweep(&measurements), 1233);
    }

    #[test]
    fn sonar_sweep_filtered_returns_5_increases() {
        let measurements = vec![607, 618, 618, 617, 647, 716, 769, 792];
        assert_eq!(sonar_sweep_filtered(&measurements), 5);
    }

    #[test]
    fn sonar_sweep_filtered_returns_amount_of_increases() {
        let measurements = buff_to_vec(&INPUT).unwrap();
        assert_eq!(sonar_sweep_filtered(&measurements), 1275);
    }
}
