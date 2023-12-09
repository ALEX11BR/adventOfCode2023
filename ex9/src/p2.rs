use std::{io::stdin, collections::VecDeque};

fn solve_line(numbers: VecDeque<i64>) -> i64 {
    let mut work = vec![numbers];
    while !work.last().unwrap().iter().all(|n| *n == 0) {
        let last_line = work.last().unwrap();
        let mut new_line = VecDeque::new();

        for i in 0..(last_line.len() - 1) {
            new_line.push_back(last_line[i + 1] - last_line[i]);
        }

        work.push(new_line);
    }

    work.last_mut().unwrap().push_front(0);
    for i in (0..(work.len() - 1)).rev() {
        let new_v = work[i][0] - work[i + 1][0];
        work[i].push_front(new_v);
    }
    
    work[0][0]
}

fn main() {
    let result = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| {
            line
                .split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap_or(0))
                .collect::<VecDeque<_>>()
        })
        .map(solve_line)
        .sum::<i64>();

    println!("{result}");
}
