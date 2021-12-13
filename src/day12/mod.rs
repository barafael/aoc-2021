use std::collections::HashSet;
use std::{collections::HashMap, str::FromStr};

pub mod problem_1;
pub mod problem_2;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Caves {
    /// Look up adjacent caves given index.
    elements: HashMap<usize, Vec<usize>>,
    names: HashMap<usize, String>,
}

impl FromStr for Caves {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = HashMap::<usize, Vec<usize>>::new();
        let mut names = HashMap::<String, usize>::new();
        let mut idx = 1;
        names.insert("start".into(), 0);
        names.insert("end".into(), usize::MAX);
        for line in s.lines() {
            let (a, b) = line.split_once('-').unwrap();
            let _ = get_or_insert_name(a, &mut idx, &mut names);
            let _ = get_or_insert_name(b, &mut idx, &mut names);
        }
        for line in s.lines() {
            let (a, b) = line.split_once('-').unwrap();
            let a_idx = names[a];
            let b_idx = names[b];
            if let Some(neighbours) = graph.get_mut(&a_idx) {
                neighbours.push(b_idx);
            } else {
                graph.insert(a_idx, vec![b_idx]);
            }
            if let Some(neighbours) = graph.get_mut(&b_idx) {
                neighbours.push(a_idx);
            } else {
                graph.insert(b_idx, vec![a_idx]);
            }
        }
        let names = names
            .into_iter()
            .map(|(name, idx)| (idx, name))
            .collect::<HashMap<usize, String>>();
        Ok(Self {
            elements: graph,
            names,
        })
    }
}

#[must_use]
fn get_or_insert_name(name: &str, idx: &mut usize, names: &mut HashMap<String, usize>) -> usize {
    if let Some(idx) = names.get(name) {
        *idx
    } else {
        let is_big_cave = !name.chars().next().unwrap().is_lowercase();
        let new_idx = if *idx % 2 == is_big_cave as usize {
            *idx + 2
        } else {
            *idx + 1
        };
        names.insert(name.to_string(), new_idx);
        *idx = new_idx;
        new_idx
    }
}

impl Caves {
    pub const fn is_start(elem: &usize) -> bool {
        *elem == 0
    }

    pub const fn is_end(elem: &usize) -> bool {
        *elem == usize::MAX
    }

    /// May not be called to get the 'size' of end.
    /// end is [`usize::MAX`], which is uneven, but 'end' is a small cave.
    pub fn is_small(elem: &usize) -> bool {
        elem % 2 == 0
    }
}

pub fn get_paths(caves: &Caves, allow_one_small_cave: bool) -> usize {
    let mut visited = HashSet::<usize>::new();
    visited.insert(0);
    count_paths_from(caves, 0, &visited, allow_one_small_cave)
}

fn count_paths_from(
    graph: &Caves,
    cave: usize,
    visited: &HashSet<usize>,
    allow_one_small_cave: bool,
) -> usize {
    if Caves::is_end(&cave) {
        return 1;
    }
    let mut count = 0;
    for nb in &graph.elements[&cave] {
        if !Caves::is_small(nb) {
            count += count_paths_from(graph, *nb, &visited.clone(), allow_one_small_cave);
        } else if !visited.contains(nb) {
            let mut visited = visited.clone();
            visited.insert(*nb);
            count += count_paths_from(graph, *nb, &visited, allow_one_small_cave);
        } else if allow_one_small_cave && !Caves::is_start(nb) {
            count += count_paths_from(graph, *nb, &visited.clone(), false);
        }
    }
    count
}

#[cfg(test)]
mod test {
    #[cfg(feature = "non_solution_test")]
    use super::Caves;
    #[cfg(feature = "non_solution_test")]
    use std::str::FromStr;

    pub const INPUT: &str = include_str!("../../input/day12.txt");

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn parses_caves() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let caves = Caves::from_str(input).unwrap();
        let expected = Caves {
            elements: [
                (6, vec![3]),
                (8, vec![4]),
                (18446744073709551615, vec![3, 4]),
                (0, vec![3, 4]),
                (3, vec![0, 6, 4, 18446744073709551615]),
                (4, vec![0, 3, 8, 18446744073709551615]),
            ]
            .iter()
            .cloned()
            .collect(),
            names: [
                (18446744073709551615, "end".into()),
                (6, "c".into()),
                (8, "d".into()),
                (0, "start".into()),
                (4, "b".into()),
                (3, "A".into()),
            ]
            .iter()
            .cloned()
            .collect(),
        };
        assert_eq!(expected, caves);
    }
}
