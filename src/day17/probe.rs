use super::TargetArea;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Probe {
    pub(crate) x: i64,
    pub(crate) y: i64,
    dx: i64,
    dy: i64,
}

impl Probe {
    pub const fn new(dx: i64, dy: i64) -> Self {
        Self { x: 0, y: 0, dx, dy }
    }

    pub fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;

        self.dx += match self.dx.cmp(&0) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        self.dy -= 1;
    }

    pub fn is_in_target_area(&self, target_area: &TargetArea) -> bool {
        (target_area.x_min..=target_area.x_max).contains(&self.x)
            && (target_area.y_min..=target_area.y_max).contains(&self.y)
    }

    pub const fn overshot(&self, target_area: &TargetArea) -> bool {
        if self.x > target_area.x_max {
            return true;
        }
        if self.y < target_area.y_min {
            return true;
        }
        false
    }
}
