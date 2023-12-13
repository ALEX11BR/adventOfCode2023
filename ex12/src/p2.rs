use itertools::Itertools;
use memoize::memoize;
use std::io::stdin;

#[memoize]
fn solve(line: Vec<char>, patterns: Vec<usize>) -> i64 {
    if patterns.is_empty() {
        line.iter().all(|c| *c != '#') as i64
    } else if patterns.iter().sum::<usize>() > line.len() {
        0
    } else if line[0] == '#' {
        let possible_match = line.iter().take(patterns[0] + 1).map(|c| *c).collect_vec();
        if possible_match.len() >= patterns[0]
            && possible_match.iter().take(patterns[0]).all(|c| *c != '.')
            && possible_match.get(patterns[0]) != Some(&'#')
        {
            solve(
                line.into_iter().skip(patterns[0] + 1).collect_vec(),
                patterns.into_iter().skip(1).collect_vec(),
            )
        } else {
            0
        }
    } else if line[0] == '?' {
        let result_1 = solve(
            line.iter().skip(1).map(|c| *c).collect_vec(),
            patterns.clone(),
        );

        let mut line = line;
        line[0] = '#';

        result_1 + solve(line, patterns)
    } else {
        solve(
            line.into_iter().skip_while(|c| *c == '.').collect_vec(),
            patterns,
        )
    }
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
        .map(|(mut line, patterns)| {
            let line_len = line.len();
            let patterns_len = patterns.len();
            line.push('?');
            (
                line.into_iter()
                    .cycle()
                    .take(5 * line_len + 4)
                    .collect_vec(),
                patterns
                    .into_iter()
                    .cycle()
                    .take(5 * patterns_len)
                    .collect_vec(),
            )
        })
        .map(|(line, patterns)| {
            solve(line, patterns)
        })
        .sum();

    println!("{result}");
}
