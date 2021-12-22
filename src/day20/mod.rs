pub mod image;
pub mod problem_1;
pub mod problem_2;
pub mod trenchmap;

use std::str::FromStr;

type Coord = (isize, isize);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Enhancement([u128; 4]);

impl FromStr for Enhancement {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut enhancement = [0u128; 4];
        let mut bits = 0;

        for (idx, ch) in input.chars().enumerate() {
            let bit_idx = idx % 128;
            let bit = (ch == '#') as u128;

            bits |= bit << bit_idx;

            if bit_idx == 127 {
                let arr_idx = idx / 128;
                enhancement[arr_idx] = bits;
                bits = 0;
            }
        }

        Ok(Self(enhancement))
    }
}

impl Enhancement {
    pub const fn is_light(&self, idx: usize) -> bool {
        let arr_idx = idx / 128;
        let bit_idx = idx % 128;
        let bit = (self.0[arr_idx] >> bit_idx) & 1;
        bit == 1
    }
}

#[cfg(test)]
mod test {
    pub const INPUT: &str = include_str!("../../input/day20.txt");
}
