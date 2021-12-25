use super::{rotation::Rotation, Vector};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Beacon {
    x: i16,
    y: i16,
    z: i16,
}

impl FromStr for Beacon {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .split(',')
            .map(|x| x.parse::<i16>().unwrap())
            .collect::<Vec<_>>();
        Ok(Self {
            x: v[0],
            y: v[1],
            z: v[2],
        })
    }
}

impl From<Beacon> for Vector {
    fn from(Beacon { x, y, z }: Beacon) -> Self {
        (x, y, z)
    }
}

impl Beacon {
    const fn n(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }

    pub const fn t((x, y, z): Vector) -> Self {
        Self::n(x, y, z)
    }

    pub fn rotate(self, rotation: Rotation) -> Self {
        Self::t(rotation.apply(self))
    }

    pub const fn offset(self, (x, y, z): Vector) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }

    pub fn abs_dist(self, other: Self) -> Vector {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        let dz = (self.z - other.z).abs();
        let min = dx.abs().min(dy.abs()).min(dz.abs()) as i16;
        let max = dx.abs().max(dy.abs()).max(dz.abs()) as i16;
        let middle = dx + dy + dz - min - max;
        (min, middle, max)
    }

    pub const fn dist(self, other: Self) -> Vector {
        (self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
