use itertools::Itertools;
use std::io::stdin;

fn check(line: &Vec<char>, hashes: &Vec<usize>, patterns: &Vec<usize>) -> i64 {
    let mut the_line = line.clone();
    for i in hashes {
        the_line[*i] = '#';
    }

    let mut the_patterns = Vec::new();
    let mut to_add = 0usize;
    for c in the_line {
        if c == '#' {
            to_add += 1;
        } else {
            if to_add > 0 {
                the_patterns.push(to_add);
                to_add = 0;
            }
        }
    }
    if to_add > 0 {
        the_patterns.push(to_add);
    }

    return (&the_patterns == patterns) as i64;
}

fn main() {
    let result: i64 = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .filter_map(|line| {
            line.split_ascii_whitespace()
                .collect_tuple::<(&str, &str)>()
                .map(|f| {
                    (
                        f.0.chars().collect_vec(),
                        f.1.split(',')
                            .filter_map(|d| d.parse::<usize>().ok())
                            .collect_vec(),
                    )
                })
        })
        .map(|(line, patterns)| {
            line
                .iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    if *c == '?' {
                        Some(i)
                    } else {
                        None
                    }
                })
                .combinations(
                    patterns.iter().sum::<usize>()
                    -
                    line
                        .iter()
                        .filter(|l| **l == '#')
                        .count()
                )
                .map(|comb| {
                    check(&line, &comb, &patterns)
                })
                .sum::<i64>()
        })
        .sum();

    println!("{result}");
}
