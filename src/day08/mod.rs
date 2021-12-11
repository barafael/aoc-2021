use std::{collections::HashSet, str::FromStr};

pub mod problem_1;
pub mod problem_2;
mod sevenseg_deduction;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Entry {
    patterns: Vec<HashSet<char>>,
    output: Vec<String>,
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_whitespace().collect::<Vec<_>>();
        if tokens.get(10).ok_or(())? != &"|" {
            return Err(());
        }
        if tokens.len() != 15 {
            return Err(());
        }
        let tokens = &mut tokens.iter();
        let patterns = tokens
            .take(10)
            .copied()
            .map(String::from)
            .map(|s| s.chars().collect())
            .collect::<Vec<HashSet<_>>>();
        let _pipe = tokens.next().ok_or(())?;
        let output = tokens
            .take(4)
            .copied()
            .map(String::from)
            .collect::<Vec<_>>();
        Ok(Self { patterns, output })
    }
}

pub fn parse_to_vec(input: &str) -> Vec<Entry> {
    input
        .trim()
        .lines()
        .map(Entry::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[cfg(test)]
mod test {
    pub const INPUT: &str = include_str!("../../input/day08.txt");

    #[cfg(feature = "non_solution_test")]
    use super::parse_to_vec;
    #[cfg(feature = "non_solution_test")]
    pub const EXAMPLE_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[cfg(feature = "non_solution_test")]
    #[test]
    fn parses_input_ok() {
        parse_to_vec(INPUT);
    }
}
