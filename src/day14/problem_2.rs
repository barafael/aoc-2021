use std::collections::HashMap;

pub fn apply_replacement(
    pairs: HashMap<[u8; 2], usize>,
    rules: &HashMap<[u8; 2], u8>,
    char_count: &mut HashMap<u8, usize>,
) -> HashMap<[u8; 2], usize> {
    let mut new_pairs = HashMap::new();
    for (pair, count) in pairs.into_iter() {
        if let Some(insert) = rules.get(&pair) {
            let key1: [u8; 2] = [pair[0], *insert];
            let key2: [u8; 2] = [*insert, pair[1]];
            let first = new_pairs.entry(key1).or_insert(count);
            *first += count;
            let second = new_pairs.entry(key2).or_insert(count);
            *second += count;
            char_count.entry(*insert).and_modify(|val| *val += count);
        } else {
            unreachable!()
        }
    }
    new_pairs
}

pub fn print_table(map: &HashMap<[u8; 2], usize>) {
    for (entry, count) in map {
        println!("{}{}: {}", entry[0] as char, entry[1] as char, count);
    }
}

#[cfg(test)]
mod tests {
    use crate::day14::{
        parse_input,
        problem_2::{apply_replacement, print_table},
        test::{EXAMPLE_INPUT, INPUT},
    };
    use std::collections::HashMap;

    #[test]
    #[ignore]
    fn performs_10_steps_on_example_input() {
        let (template, rules) = parse_input(EXAMPLE_INPUT);
        let rules = rules
            .into_iter()
            .map(|(key, value)| (key.as_bytes().try_into().unwrap(), value as u8))
            .collect::<HashMap<_, _>>();
        let mut pairs = HashMap::<[u8; 2], usize>::new();
        let mut counts = HashMap::<u8, usize>::new();
        for byte_pair in template.as_bytes().windows(2) {
            pairs
                .entry(byte_pair.try_into().unwrap())
                .and_modify(|v| *v += 1)
                .or_insert(1);
            counts
                .entry(byte_pair[0])
                .and_modify(|val| *val += 1)
                .or_insert(1);
        }
        let b = template.chars().last();
        counts
            .entry(b.unwrap() as u8)
            .and_modify(|v| *v += 1)
            .or_insert(1);

        let mut pairs = pairs;
        for _i in 0..10 {
            pairs = apply_replacement(pairs, &rules, &mut counts);
        }
        let max = dbg!(counts.values().max().unwrap());
        let min = dbg!(counts.values().min().unwrap());
        let sum = pairs.values().sum::<usize>() + 1;
        //assert_eq!(3073, sum);
        assert_eq!(2188189693529, max - min);
    }
}
