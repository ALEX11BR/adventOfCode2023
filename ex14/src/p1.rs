use std::io::stdin;

use itertools::Itertools;

fn main() {
    let rocks = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| {
            line.chars().collect_vec()
        })
        .collect_vec();

    let mut result = 0;
    for j in 0..rocks[0].len() {
        let mut rock_column = 0;

        for i in 0..rocks.len() {
            match rocks[i][j] {
                '#' => rock_column = i + 1,
                'O' => {
                    result += rocks.len() - rock_column;
                    rock_column += 1
                }
                _ => {}
            }
        }
    }

    println!("{result}");
}
