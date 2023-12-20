use std::{collections::HashMap, error::Error, io::stdin};

use itertools::Itertools;

type Range = (i64, i64);
type ItemRange = HashMap<char, Range>;

#[derive(Clone, Debug)]
struct Condition {
    compare_to: char,
    // minimum and maximum
    condition: Range,
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

    fn difference_from_range(&self, range: Range) -> Range {
        let opposite = if self.condition.0 == i64::MIN {
            (self.condition.1 - 1, i64::MAX)
        } else {
            (i64::MIN, self.condition.0 + 1)
        };

        range_intersection(range, opposite)
    }
}

fn numbers_in_range(range: Range) -> i64 {
    range.1 - range.0 - 1
}

fn range_intersection(a: Range, b: Range) -> Range {
    (a.0.max(b.0), a.1.min(b.1))
}

fn range_not_empty(a: Range) -> bool {
    a.0 < a.1 - 1
}

fn solve(range: ItemRange, state_str: &str, workflows: &HashMap<String, Vec<Condition>>) -> i64 {
    if let Some(workflow) = workflows.get(state_str) {
        let mut range = range;
        let mut result = 0;

        for condition in workflow {
            let modifiable_range = range[&condition.compare_to];

            let branching_range = range_intersection(modifiable_range, condition.condition);
            let remaining_range = condition.difference_from_range(modifiable_range);

            if range_not_empty(branching_range) {
                let mut branching = range.clone();
                branching
                    .get_mut(&condition.compare_to)
                    .map(|r| *r = branching_range);

                result += solve(branching, &condition.next_state, workflows)
            }

            if !range_not_empty(remaining_range) {
                break;
            }
            range
                .get_mut(&condition.compare_to)
                .map(|r| *r = remaining_range);
        }

        result
    } else if state_str == "A" {
        numbers_in_range(range[&'x'])
            * numbers_in_range(range[&'m'])
            * numbers_in_range(range[&'a'])
            * numbers_in_range(range[&'s'])
    } else {
        0
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

    let result = solve(
        HashMap::from([
            ('x', (0, 4001)),
            ('m', (0, 4001)),
            ('a', (0, 4001)),
            ('s', (0, 4001)),
        ]),
        "in",
        &workflows,
    );
    println!("{result}");

    Ok(())
}
