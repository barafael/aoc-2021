use std::cmp;
use std::collections::HashSet;
use std::str::FromStr;

use super::{Coord, Enhancement};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Image {
    light_pixels: HashSet<Coord>,
    is_outside_light: bool,
    top: isize,
    left: isize,
    bottom: isize,
    right: isize,
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut light_pixels = HashSet::new();
        let mut max_width = 0;
        let mut max_height = 0;

        for (line, y) in s.lines().zip(0..) {
            for (ch, x) in line.chars().zip(0..) {
                max_width = cmp::max(max_width, x);

                match ch {
                    '#' => {
                        light_pixels.insert((x, y));
                    }
                    '.' => (),
                    _ => return Err(()),
                }
            }

            max_height = cmp::max(max_height, y);
        }

        Ok(Self {
            light_pixels,
            is_outside_light: false,
            top: 0,
            left: 0,
            bottom: max_height,
            right: max_width,
        })
    }
}

impl Image {
    pub const fn is_coord_inbound(&self, coord: &Coord) -> bool {
        let check_x = coord.0 >= self.left && coord.0 <= self.right;
        let check_y = coord.1 >= self.top && coord.1 <= self.bottom;
        check_x && check_y
    }

    pub fn get_enhancement_idx(&self, x: isize, y: isize) -> usize {
        let mut idx = 0;

        let coords = [
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x + 1, y),
            (x, y),
            (x - 1, y),
            (x + 1, y - 1),
            (x, y - 1),
            (x - 1, y - 1),
        ];

        for (i, coord) in coords.into_iter().enumerate() {
            let bit = if self.is_coord_inbound(&coord) {
                self.light_pixels.contains(&coord) as usize
            } else {
                self.is_outside_light as usize
            };

            idx |= bit << i;
        }

        idx
    }

    pub fn padding_one(&mut self) {
        self.top -= 1;
        self.left -= 1;
        self.right += 1;
        self.bottom += 1;

        if !self.is_outside_light {
            return;
        }

        let top_iter = (self.left..=self.right).map(|x| (x, self.top));
        let bottom_iter = (self.left..=self.right).map(|x| (x, self.bottom));
        let left_iter = ((self.top + 1)..self.bottom).map(|y| (self.left, y));
        let right_iter = ((self.top + 1)..self.bottom).map(|y| (self.right, y));

        let light_coords = top_iter
            .chain(bottom_iter)
            .chain(left_iter)
            .chain(right_iter);

        self.light_pixels.extend(light_coords);
    }

    pub fn apply_enhancement(&mut self, enhancement: &Enhancement) {
        self.padding_one();

        let clone = Self {
            light_pixels: std::mem::take(&mut self.light_pixels),
            is_outside_light: self.is_outside_light,
            top: self.top,
            left: self.left,
            right: self.right,
            bottom: self.bottom,
        };

        for x in self.left..=self.right {
            for y in self.top..=self.bottom {
                let idx = clone.get_enhancement_idx(x, y);
                if enhancement.is_light(idx) {
                    self.light_pixels.insert((x, y));
                }
            }
        }

        if enhancement.is_light(0) {
            self.is_outside_light = !self.is_outside_light;
        }
    }

    pub fn count_light(&self) -> u64 {
        self.light_pixels.len() as u64
    }
}
