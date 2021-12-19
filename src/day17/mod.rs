use std::str::FromStr;

pub mod probe;
pub mod problem_1;
pub mod problem_2;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct TargetArea {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl FromStr for TargetArea {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches("target area: ").trim();
        let (x, y) = s.split_once(", ").unwrap();
        let x = x.trim_start_matches("x=");
        let (x_min, x_max) = x.split_once("..").unwrap();
        let y = y.trim_start_matches("y=");
        let (y_min, y_max) = y.split_once("..").unwrap();
        let x_min = x_min.parse().unwrap();
        let x_max = x_max.parse().unwrap();
        let y_min = y_min.parse().unwrap();
        let y_max = y_max.parse().unwrap();
        assert!(x_min < x_max);
        assert!(y_min < y_max);
        Ok(Self {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }
}

#[cfg(test)]
mod test {
    use super::TargetArea;
    use std::str::FromStr;

    pub const INPUT: &str = include_str!("../../input/day17.txt");

    #[test]
    pub fn parses_input_ok() {
        let target_area = TargetArea::from_str(INPUT).unwrap();
        assert_eq!(
            TargetArea {
                x_min: 139,
                x_max: 187,
                y_min: -148,
                y_max: -89,
            },
            target_area
        );
    }
}
