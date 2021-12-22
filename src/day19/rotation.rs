use super::{
    coord::{Coord, Sign},
    Vector,
};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Rotation(Sign, Sign, Sign);

impl Rotation {
    pub fn rotations() -> impl Iterator<Item = Self> {
        let o = Rotation;
        let px = Sign::Pos(Coord::X);
        let py = Sign::Pos(Coord::Y);
        let pz = Sign::Pos(Coord::Z);
        let nx = Sign::Neg(Coord::X);
        let ny = Sign::Neg(Coord::Y);
        let nz = Sign::Neg(Coord::Z);
        [
            o(px, py, pz),
            o(nx, py, pz),
            o(px, ny, pz),
            o(px, py, nz),
            o(nx, ny, pz),
            o(nx, py, nz),
            o(px, ny, nz),
            o(nx, ny, nz),
        ]
        .into_iter()
        .flat_map(move |order| {
            let Rotation(x, y, z) = order;
            [
                o(x, y, z),
                o(x, z, y),
                o(y, x, z),
                o(y, z, x),
                o(z, x, y),
                o(z, y, x),
            ]
            .into_iter()
        })
    }

    pub fn apply<I: Into<Vector>>(self, b: I) -> Vector {
        let b = b.into();
        (self.0.apply(b), self.1.apply(b), self.2.apply(b))
    }
}
