pub mod parser;
pub mod problem_1;
pub mod problem_2;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Packet<T> {
    version: u8,
    type_id: u8,
    payload: T,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Literal(u64);

#[cfg(test)]
mod test {
    pub const INPUT: &str = include_str!("../../input/day16.txt");

    pub const EXAMPLE_1: &str = "8A004A801A8002F478";
    pub const EXAMPLE_2: &str = "620080001611562C8802118E34";
    pub const EXAMPLE_3: &str = "C0015000016115A2E0802F182340";
    pub const EXAMPLE_4: &str = "A0016C880162017C3686B18A3D4780";
}
