use itertools::Itertools;
use std::collections::HashMap;

pub fn apply_replacements_naive(template: String, rules: &HashMap<String, char>) -> String {
    let mut result = template
        .chars()
        .tuple_windows::<(char, char)>()
        .map(|(a, b)| {
            let key = format!("{}{}", a, b);
            if let Some(value) = rules.get(&key) {
                format!("{}{}", a, value)
            } else {
                "".into()
            }
        })
        .collect::<String>();
    result.push(template.chars().last().unwrap());
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::day14::{
        parse_input,
        problem_1::apply_replacements_naive,
        test::{EXAMPLE_INPUT, INPUT},
    };

    #[test]
    fn performs_1_step_on_example_input() {
        let (template, rules) = parse_input(EXAMPLE_INPUT);
        assert_eq!(
            "NCNBCHB".to_string(),
            apply_replacements_naive(template, &rules)
        );
    }

    #[test]
    fn performs_10_steps_on_example_input() {
        let (template, rules) = parse_input(EXAMPLE_INPUT);
        let mut input = template;
        for _i in 0..10 {
            input = apply_replacements_naive(input, &rules);
        }
        assert_eq!(3073, input.len());
        let mut counters = HashMap::<char, usize>::new();
        for c in input.chars() {
            let count = counters.entry(c).or_default();
            *count += 1;
        }
        let max = counters.values().max().unwrap();
        let min = counters.values().min().unwrap();
        assert_eq!(1588, max - min);
        dbg!(input);
    }

    #[test]
    fn performs_10_steps_on_input() {
        let (template, rules) = parse_input(INPUT);
        let mut input = template;
        for _i in 0..10 {
            input = apply_replacements_naive(input, &rules);
        }
        let mut counters = HashMap::<char, usize>::new();
        for c in input.chars() {
            let count = counters.entry(c).or_default();
            *count += 1;
        }
        let max = counters.values().max().unwrap();
        let min = counters.values().min().unwrap();
        assert_eq!(2549, max - min);
    }
}
