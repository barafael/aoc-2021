use std::collections::HashMap;

pub mod problem_1;
pub mod problem_2;

pub fn parse_input(input: &str) -> (String, HashMap<String, char>) {
    let mut lines = input.lines();
    let template = lines.next().unwrap().to_string();
    let _ = lines.next().unwrap();

    let mut map = HashMap::new();
    for line in lines {
        let (pattern, middle) = line.split_once(" -> ").unwrap();
        assert_eq!(1, middle.len());
        map.insert(pattern.to_string(), middle.chars().next().unwrap());
    }
    (template, map)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::parse_input;

    pub const INPUT: &str = include_str!("../../input/day14.txt");
    pub const EXAMPLE_INPUT: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn parses_input() {
        let (template, rules) = parse_input(EXAMPLE_INPUT);
        assert_eq!("NNCB".to_string(), template);
        assert_eq!(
            [
                ("HN".to_string(), 'C'),
                ("NC".to_string(), 'B'),
                ("BC".to_string(), 'B'),
                ("BB".to_string(), 'N'),
                ("HH".to_string(), 'N'),
                ("NN".to_string(), 'C'),
                ("BH".to_string(), 'H'),
                ("BN".to_string(), 'B'),
                ("CC".to_string(), 'N'),
                ("CB".to_string(), 'H'),
                ("CN".to_string(), 'C'),
                ("CH".to_string(), 'B'),
                ("HC".to_string(), 'B'),
                ("HB".to_string(), 'C'),
                ("NB".to_string(), 'B'),
                ("NH".to_string(), 'C'),
            ]
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>(),
            rules
        );
    }
}
