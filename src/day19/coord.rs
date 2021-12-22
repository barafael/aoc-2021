use super::Vector;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Coord {
    X,
    Y,
    Z,
}

impl Coord {
    fn apply<I: Into<Vector>>(self, b: I) -> i16 {
        let b = b.into();
        match self {
            Coord::X => b.0,
            Coord::Y => b.1,
            Coord::Z => b.2,
        }
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Sign {
    Neg(Coord),
    Pos(Coord),
}

impl Sign {
    pub fn apply<I: Into<Vector>>(self, b: I) -> i16 {
        let b = b.into();
        match self {
            Sign::Neg(c) => -c.apply(b),
            Sign::Pos(c) => c.apply(b),
        }
    }
}
