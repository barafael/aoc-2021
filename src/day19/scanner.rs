use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use super::{beacon::Beacon, rotation::Rotation, Vector};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Scanner {
    pub(crate) beacons: Vec<Beacon>,
    pub(crate) coords: Vector,
    pub(crate) abs_dist: HashMap<Beacon, HashSet<Vector>>,
}

impl FromStr for Scanner {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let beacons = s
            .lines()
            .skip(1)
            .map(|l| l.parse::<Beacon>().unwrap())
            .collect::<Vec<_>>();
        Ok(Self::new(beacons).with_abs_dist())
    }
}

impl Scanner {
    fn with_abs_dist(mut self) -> Self {
        self.abs_dist = self
            .beacons
            .iter()
            .copied()
            .map(|b| {
                (
                    b,
                    self.beacons
                        .iter()
                        .copied()
                        .map(|b2| b.abs_dist(b2))
                        .filter(|d| *d != (0, 0, 0))
                        .collect(),
                )
            })
            .collect();
        self
    }

    const fn with_coords(mut self, coords: Vector) -> Self {
        self.coords = coords;
        self
    }

    fn new(beacons: Vec<Beacon>) -> Self {
        Self {
            beacons,
            coords: (0, 0, 0),
            abs_dist: HashMap::new(),
        }
        .with_abs_dist()
    }

    pub fn adjust_to_other_space(&self, rotation: Rotation, offset: Vector) -> Self {
        let beacons = self
            .beacons
            .iter()
            .copied()
            .map(|r| r.rotate(rotation).offset(offset))
            .collect::<Vec<_>>();
        Self::new(beacons).with_coords(offset)
    }

    pub fn overlaps(&self, other: &Self) -> Option<(Rotation, Vector)> {
        for &beacon in &self.beacons {
            let self_distances = self.abs_dist.get(&beacon).unwrap();
            for &other_beacon in &other.beacons {
                let other_distances = other.abs_dist.get(&other_beacon).unwrap();
                let overlap = self_distances.intersection(other_distances).count();
                if overlap >= 11 {
                    let self_dists = self
                        .beacons
                        .iter()
                        .copied()
                        .map(|x| x.dist(beacon))
                        .filter(|x| *x != (0, 0, 0))
                        .collect::<HashSet<_>>();
                    for rotation in Rotation::rotations() {
                        let other_dists = other
                            .beacons
                            .iter()
                            .copied()
                            .map(|x| rotation.apply(x.dist(other_beacon)))
                            .filter(|x| *x != (0, 0, 0))
                            .collect::<HashSet<_>>();
                        let intersection = self_dists.intersection(&other_dists).count();
                        if intersection >= 11 {
                            return Some((rotation, beacon.dist(other_beacon.rotate(rotation))));
                        }
                    }
                }
            }
        }
        None
    }
}
