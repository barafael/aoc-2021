use itertools::Itertools;
use std::{collections::HashMap, iter::once};

type PairMap = HashMap<char, HashMap<char, usize>>;

/// Applies a rule to each pair, inserting the two resulting pairs in the new `PairMap`.
pub fn step(pairs: PairMap, rules: &HashMap<(char, char), char>) -> PairMap {
    // TODO simplify with entry API of hashmap
    let mut new_pairs = PairMap::new();
    for (fst, snds) in pairs {
        for (snd, count) in snds {
            let insert = if let Some(i) = rules.get(&(fst, snd)) {
                i
            } else {
                if snd == '@' {
                    if let Some(entry) = new_pairs.get_mut(&fst) {
                        entry.insert('@', 1);
                    } else {
                        new_pairs.insert(fst, [('@', 1)].into_iter().collect());
                    }
                }
                continue;
            };
            // (fst, insert)
            if let Some(entry) = new_pairs.get_mut(&fst) {
                if let Some(entry) = entry.get_mut(insert) {
                    *entry += count;
                } else {
                    entry.insert(*insert, count);
                }
            } else {
                new_pairs.insert(fst, [(*insert, count)].into_iter().collect());
            }
            // (insert, snd)
            if let Some(entry) = new_pairs.get_mut(insert) {
                if let Some(entry) = entry.get_mut(&snd) {
                    *entry += count;
                } else {
                    entry.insert(snd, count);
                }
            } else {
                new_pairs.insert(*insert, [(snd, count)].into_iter().collect());
            }
        }
    }
    new_pairs
}

pub fn count_occurrences(pairs: &PairMap) -> HashMap<char, usize> {
    let mut counts = HashMap::<char, usize>::new();
    for (char, followers) in pairs {
        let sum = followers.iter().map(|(_, c)| *c).sum();
        counts.insert(*char, sum);
    }
    counts
}

pub fn calculate_score(occurrences: &HashMap<char, usize>) -> usize {
    let max = occurrences.iter().map(|(_, count)| count).max().unwrap();
    let min = occurrences.iter().map(|(_, count)| count).min().unwrap();
    max - min
}

pub fn pairs_from_str(string: &str) -> PairMap {
    let mut map = PairMap::new();
    for (a, b) in string.chars().chain(once('@')).tuple_windows() {
        if let Some(entry) = map.get_mut(&a) {
            if let Some(entry) = entry.get_mut(&b) {
                *entry += 1;
            } else {
                entry.insert(b, 1);
            }
        } else {
            map.insert(a, [(b, 1)].into_iter().collect());
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::{pairs_from_str, step};
    use crate::day14::{
        parse_input,
        problem_2::{count_occurrences, PairMap},
        test::{EXAMPLE_INPUT, INPUT},
    };
    use std::collections::HashMap;

    #[test]
    fn applies_first_step() {
        let (initial, rules) = parse_input(EXAMPLE_INPUT);
        let rules = rules
            .iter()
            .map(|(pair, c)| ((pair.as_bytes()[0] as char, pair.as_bytes()[1] as char), *c))
            .collect::<HashMap<_, _>>();
        let pairs = pairs_from_str(&initial);
        let result = step(pairs, &rules);
        let expected = [
            ('N', [('B', 1), ('C', 1)].into_iter().collect()),
            ('B', [('@', 1), ('C', 1)].into_iter().collect()),
            ('H', [('B', 1)].into_iter().collect()),
            ('C', [('N', 1), ('H', 1)].into_iter().collect()),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();
        assert_eq!(expected, result);
    }

    #[test]
    fn applies_10_steps() {
        let (initial, rules) = parse_input(EXAMPLE_INPUT);
        let rules = rules
            .iter()
            .map(|(pair, c)| ((pair.as_bytes()[0] as char, pair.as_bytes()[1] as char), *c))
            .collect::<HashMap<_, _>>();
        let mut pairs = pairs_from_str(&initial);
        for _i in 1..=10 {
            pairs = step(pairs, &rules);
            if _i == 5 {
                let len: usize = count_occurrences(&pairs).iter().map(|(_, s)| *s).sum();
                assert_eq!(97, len);
            }
        }
        let occ = count_occurrences(&pairs);
        let max = occ.iter().map(|(_, count)| count).max().unwrap();
        let min = occ.iter().map(|(_, count)| count).min().unwrap();
        assert_eq!(1588, max - min);
    }

    #[test]
    fn applies_40_steps() {
        let (initial, rules) = parse_input(EXAMPLE_INPUT);
        let rules = rules
            .iter()
            .map(|(pair, c)| ((pair.as_bytes()[0] as char, pair.as_bytes()[1] as char), *c))
            .collect::<HashMap<_, _>>();
        let mut pairs = pairs_from_str(&initial);
        for _i in 1..=40 {
            pairs = step(pairs, &rules);
        }
        let occ = count_occurrences(&pairs);
        let max = occ.iter().map(|(_, count)| count).max().unwrap();
        let min = occ.iter().map(|(_, count)| count).min().unwrap();
        assert_eq!(2188189693529, max - min);
    }

    #[test]
    fn computes_solution() {
        let (initial, rules) = parse_input(INPUT);
        let rules = rules
            .iter()
            .map(|(pair, c)| ((pair.as_bytes()[0] as char, pair.as_bytes()[1] as char), *c))
            .collect::<HashMap<_, _>>();
        let mut pairs = pairs_from_str(&initial);
        for _i in 1..=40 {
            pairs = step(pairs, &rules);
        }
        let occ = count_occurrences(&pairs);
        let max = occ.iter().map(|(_, count)| count).max().unwrap();
        let min = occ.iter().map(|(_, count)| count).min().unwrap();
        assert_eq!(2516901104210, max - min);
    }

    #[test]
    fn creates_initial_map() {
        let (initial, _rules) = parse_input(EXAMPLE_INPUT);
        let map = pairs_from_str(&initial);
        let expected: PairMap = [
            ('N', [('C', 1), ('N', 1)].into_iter().collect()),
            ('B', [('@', 1)].into_iter().collect()),
            ('C', [('B', 1)].into_iter().collect()),
        ]
        .into_iter()
        .collect();
        assert_eq!(expected, map);
    }

    #[test]
    fn creates_larger_initial_map() {
        let map = pairs_from_str("NCNBCHB".into());
        let expected: PairMap = [
            ('N', [('C', 1), ('B', 1)].into_iter().collect()),
            ('C', [('N', 1), ('H', 1)].into_iter().collect()),
            ('H', [('B', 1)].into_iter().collect()),
            ('B', [('C', 1), ('@', 1)].into_iter().collect()),
        ]
        .into_iter()
        .collect();
        assert_eq!(expected, map);
    }

    #[test]
    fn creates_even_larger_initial_map() {
        let map = pairs_from_str("NBBBCNCCNBBNBNBBCHBHHBCHB".into());
        let expected: PairMap = [
            ('N', [('C', 1), ('B', 4)].into_iter().collect()),
            (
                'B',
                [('B', 4), ('C', 3), ('N', 2), ('H', 1), ('@', 1)]
                    .into_iter()
                    .collect(),
            ),
            ('C', [('N', 2), ('C', 1), ('H', 2)].into_iter().collect()),
            ('H', [('B', 3), ('H', 1)].into_iter().collect()),
        ]
        .into_iter()
        .collect();
        assert_eq!(expected, map);
    }
}
