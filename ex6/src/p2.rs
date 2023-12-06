use std::io::stdin;

fn main() {
    let numbers = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| {
            line
                .split(':')
                .nth(1)
                .unwrap_or("")
                .split_ascii_whitespace()
                .flat_map(|s| s.chars())
                .collect::<String>()
                .parse::<i64>().unwrap_or(-1)
        })
        .collect::<Vec<_>>();

    let time_f = numbers[0] as f64;
    let distance_f = numbers[1] as f64;

    let end_hold = (((-time_f - (time_f * time_f - 4f64 * distance_f).sqrt()) / -2f64).ceil() as i64).max(0);
    let start_hold = (((-time_f + (time_f * time_f - 4f64 * distance_f).sqrt()) / -2f64).ceil() as i64).min(numbers[0]);

    let result = end_hold - start_hold;

    println!("{result}");
}
