use std::{io::stdin, iter::zip};

fn main() {
    let arrays = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| {
            line
                .split(':')
                .nth(1)
                .unwrap_or("")
                .split_ascii_whitespace()
                .map(|number| number.parse::<i64>().unwrap_or(-1))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let result = zip(&arrays[0], &arrays[1])
        .map(|(time, distance)| {
            let time_f = *time as f64;
            let distance_f = *distance as f64;

            let end_hold = (((-time_f - (time_f * time_f - 4f64 * distance_f).sqrt()) / -2f64).ceil() as i64).max(0);
            let start_hold = (((-time_f + (time_f * time_f - 4f64 * distance_f).sqrt()) / -2f64).ceil() as i64).min(*time);

            end_hold - start_hold
        })
        .fold(1, |a, b| a * b);

    println!("{result}");
}
