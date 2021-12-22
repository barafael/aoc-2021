pub mod beacon;
pub mod coord;
pub mod problem_1;
pub mod problem_2;
pub mod rotation;
pub mod scanner;

use self::beacon::Beacon;
use self::scanner::Scanner;
use std::str::FromStr;

pub type Vector = (i16, i16, i16);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Scanners {
    scanners: Vec<Scanner>,
}

impl FromStr for Scanners {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scanners = s
            .split("\n\n")
            .map(|b| b.parse::<Scanner>().unwrap())
            .collect::<Vec<_>>();
        Ok(Self { scanners })
    }
}

impl Scanners {
    pub fn max_manhattan_distance(&self) -> usize {
        let mut max_dist = 0;
        for l in &self.scanners {
            let lb = Beacon::t(l.coords);
            for r in &self.scanners {
                let rb = Beacon::t(r.coords);
                let (x, y, z) = lb.abs_dist(rb);
                let manhattan_distance = x.abs() as usize + y.abs() as usize + z.abs() as usize;
                if manhattan_distance > max_dist {
                    max_dist = manhattan_distance;
                }
            }
        }
        max_dist
    }

    pub fn rotate_all(&self) -> Self {
        let mut scanners = Vec::with_capacity(self.scanners.len());
        let mut scanned = vec![false; self.scanners.len()];
        scanners.push(self.scanners[0].clone());
        scanned[0] = true;

        while !scanned.iter().all(|x| *x) {
            for (i, scanner) in self.scanners.iter().enumerate() {
                if scanned[i] {
                    continue;
                }
                for final_scanner in &scanners {
                    if let Some((rotation, vector)) = final_scanner.overlaps(scanner) {
                        scanners.push(scanner.adjust_to_other_space(rotation, vector));
                        scanned[i] = true;
                        break;
                    }
                }
            }
        }
        Self { scanners }
    }
}

#[cfg(test)]
mod test {
    pub const INPUT: &str = include_str!("../../input/day19.txt");
}
