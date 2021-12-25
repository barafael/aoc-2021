pub mod problem_1;
pub mod problem_2;

use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

use coordinates::Xyz;
use cube::{AntiCuboid, Cuboid, PosiCuboid};

mod coordinates;
mod cube;

pub enum Instr {
    On(Xyz, Xyz),
    Off(Xyz, Xyz),
}

pub fn parse_input(input: &str, range: Option<&RangeInclusive<i32>>) -> Vec<Instr> {
    let re = regex::Regex::new(
        r"(?P<action>on|off) x=(?P<xl>-?\d+)\.{2}(?P<xu>-?\d+),y=(?P<yl>-?\d+)\.{2}(?P<yu>-?\d+),z=(?P<zl>-?\d+)\.{2}(?P<zu>-?\d+)",
    )
    .unwrap();

    re.captures_iter(input)
        .filter_map(|c| {
            let (action, xl, xu, yl, yu, zl, zu) = (
                c["action"].as_bytes(),
                c["xl"].parse::<i32>().unwrap(),
                c["xu"].parse::<i32>().unwrap(),
                c["yl"].parse::<i32>().unwrap(),
                c["yu"].parse::<i32>().unwrap(),
                c["zl"].parse::<i32>().unwrap(),
                c["zu"].parse::<i32>().unwrap(),
            );

            let (xl, xu) = (min(xl, xu), max(xl, xu));
            let (yl, yu) = (min(yl, yu), max(yl, yu));
            let (zl, zu) = (min(zl, zu), max(zl, zu));

            let from = Xyz::new(xl, yl, zl);
            let to = Xyz::new(xu, yu, zu);

            if let Some(range) = range {
                if from.x() < *range.start()
                    || to.x() > *range.end()
                    || from.y() < *range.start()
                    || to.y() > *range.end()
                    || from.z() < *range.start()
                    || to.z() > *range.end()
                {
                    return None;
                }
            }

            Some(match action {
                b"on" => Instr::On(from, to),
                b"off" => Instr::Off(from, to),
                _ => unreachable!(),
            })
        })
        .collect()
}

pub fn solve(reboot_steps: &[Instr]) -> usize {
    let mut posi_cuboids: Vec<PosiCuboid> = Vec::new();
    let mut anti_cuboids: Vec<AntiCuboid> = Vec::new();

    for s in reboot_steps {
        match s {
            Instr::On(from, to) => {
                let cuboid_to_add = PosiCuboid::new(from, to);

                let (mut posi_adj_cuboids, mut anti_adj_cuboids) =
                    get_adjustment_cuboids(&cuboid_to_add, &posi_cuboids, &anti_cuboids);

                posi_cuboids.push(cuboid_to_add);
                posi_cuboids.append(&mut posi_adj_cuboids);
                anti_cuboids.append(&mut anti_adj_cuboids);
            }

            Instr::Off(from, to) => {
                let reference_anti_cuboid = AntiCuboid::new(from, to);

                let (mut posi_adj_cuboids, mut anti_adj_cuboids) =
                    get_adjustment_cuboids(&reference_anti_cuboid, &posi_cuboids, &anti_cuboids);

                posi_cuboids.append(&mut posi_adj_cuboids);
                anti_cuboids.append(&mut anti_adj_cuboids);
            }
        }
    }

    let x: usize = posi_cuboids.iter().map(|c| c.area() as usize).sum();
    let y: usize = anti_cuboids.iter().map(|c| c.area() as usize).sum();

    x - y
}

pub fn get_adjustment_cuboids(
    rc: &impl Cuboid,
    posi: &[PosiCuboid],
    anti: &[AntiCuboid],
) -> (Vec<PosiCuboid>, Vec<AntiCuboid>) {
    let posi_cuboids = anti
        .iter()
        .filter_map(|ac| ac.make_adjustment_cuboid(rc))
        .collect::<Vec<_>>();

    let anti_cuboids = posi
        .iter()
        .filter_map(|pc| pc.make_adjustment_cuboid(rc))
        .collect::<Vec<_>>();

    (posi_cuboids, anti_cuboids)
}

#[cfg(test)]
mod tests {
    pub const INPUT: &str = include_str!("../../input/day22.txt");
}
