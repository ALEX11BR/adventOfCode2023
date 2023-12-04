use std::io::stdin;

fn main() {
    let result = stdin()
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
        .map(|num_matches| {
            if num_matches > 0 {
                2i32.pow((num_matches - 1) as u32)
            } else {
                0
            }
        })
        .sum::<i32>();

    println!("{result}");
}
