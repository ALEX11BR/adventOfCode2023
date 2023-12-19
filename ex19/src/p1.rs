use std::{collections::HashMap, error::Error, io::stdin};

use itertools::Itertools;

type Item = HashMap<char, i64>;

#[derive(Clone, Debug)]
struct Condition {
    compare_to: char,
    // minimum and maximum
    condition: (i64, i64),
    next_state: String,
}
impl Condition {
    fn from_str(s: &str) -> Self {
        let instructions = s.split(':').collect_vec();

        if instructions.len() == 1 {
            Self {
                compare_to: 'x',
                condition: (i64::MIN, i64::MAX),
                next_state: instructions[0].to_owned(),
            }
        } else {
            let conditions = instructions[0].split(&['<', '>']).collect_vec();

            Self {
                compare_to: conditions[0].chars().next().unwrap_or('?'),
                condition: if instructions[0].chars().nth(1) == Some('<') {
                    (i64::MIN, conditions[1].parse().unwrap_or(i64::MAX))
                } else {
                    (conditions[1].parse().unwrap_or(i64::MIN), i64::MAX)
                },
                next_state: instructions[1].to_owned(),
            }
        }
    }

    fn get_next_state<'a>(&'a self, item: &Item) -> Option<&'a str> {
        if let Some(value_to_test) = item.get(&self.compare_to) {
            if *value_to_test > self.condition.0 && *value_to_test < self.condition.1 {
                Some(&self.next_state)
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let workflow_regex = regex::Regex::new(r#"([a-z]+)\{(.+)\}"#)?;

    let mut lines = stdin().lines().filter_map(|result_line| result_line.ok());

    let mut workflows = HashMap::new();

    while let Some(workflow_str) = lines
        .next()
        .and_then(|s| if s.is_empty() { None } else { Some(s) })
    {
        let caps = workflow_regex.captures(&workflow_str).ok_or("")?;

        workflows.insert(
            caps[1].to_owned(),
            caps[2].split(',').map(Condition::from_str).collect_vec(),
        );
    }

    let result: i64 = lines
        .map(|line| {
            let line = line.strip_prefix("{").unwrap_or(&line);
            let line = line.strip_suffix("}").unwrap_or(line);

            line.split(',')
                .filter_map(|attr| {
                    attr.split('=')
                        .collect_tuple::<(&str, &str)>()
                        .map(|(key, value)| {
                            (
                                key.chars().next().unwrap_or('?'),
                                value.parse::<i64>().unwrap_or(0),
                            )
                        })
                })
                .collect::<Item>()
        })
        .map(|item| {
            let mut state = "in";

            while let Some(workflow) = workflows.get(state) {
                state = workflow
                    .iter()
                    .find_map(|cond| cond.get_next_state(&item))
                    .unwrap_or("?");
            }

            if state == "A" {
                item.values().sum::<i64>()
            } else {
                0
            }
        })
        .sum();
    println!("{result}");

    Ok(())
}
