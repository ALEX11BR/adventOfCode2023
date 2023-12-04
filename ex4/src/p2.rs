use std::io::stdin;

fn main() {
    let num_matches: Vec<i32> = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| {
            let fields: Vec<_> = line.split(':').collect();
            let mut numbers: Vec<Vec<i32>> = fields[1]
                .split('|')
                .map(|field| {
                    field
                        .split_ascii_whitespace()
                        .map(|s| s.parse::<i32>().unwrap_or(0))
                        .collect()
                })
                .collect();
            numbers[1].sort();
            numbers[0]
                .iter()
                .map(|n| {
                    if let Ok(_) = numbers[1].binary_search(n) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i32>()
        })
        .collect();

    let mut num_cards = vec![1; num_matches.len()];

    for i in (0..num_cards.len()).rev() {
        for j in 1..=num_matches[i] {
            num_cards[i] += num_cards[i + j as usize];
        }
    }

    println!("{}", num_cards.iter().sum::<i32>());
}
