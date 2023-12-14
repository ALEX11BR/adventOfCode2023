use std::io::stdin;

use itertools::Itertools;

fn rotate_cycle(rocks: &mut Vec<Vec<char>>) {
    // North
    for j in 0..rocks[0].len() {
        let mut rock_i = 0;

        for i in 0..rocks.len() {
            match rocks[i][j] {
                '#' => rock_i = i + 1,
                'O' => {
                    rocks[rock_i][j] = 'O';
                    if rock_i != i {
                        rocks[i][j] = '.';
                    }
                    rock_i += 1;
                }
                _ => {}
            }
        }
    }

    // West
    for i in 0..rocks.len() {
        let mut rock_j = 0;

        for j in 0..rocks[0].len() {
            match rocks[i][j] {
                '#' => rock_j = j + 1,
                'O' => {
                    rocks[i][rock_j] = 'O';
                    if rock_j != j {
                        rocks[i][j] = '.';
                    }
                    rock_j += 1;
                }
                _ => {}
            }
        }
    }

    // South
    for j in 0..rocks[0].len() {
        let mut rock_i = rocks.len() - 1;

        for i in (0..rocks.len()).rev() {
            match rocks[i][j] {
                '#' => rock_i = i.saturating_sub(1),
                'O' => {
                    rocks[rock_i][j] = 'O';
                    if rock_i != i {
                        rocks[i][j] = '.';
                    }
                    rock_i = rock_i.saturating_sub(1);
                }
                _ => {}
            }
        }
    }
    
    // East
    for i in 0..rocks.len() {
        let mut rock_j = rocks[0].len() - 1;

        for j in (0..rocks[0].len()).rev() {
            match rocks[i][j] {
                '#' => rock_j = j.saturating_sub(1),
                'O' => {
                    rocks[i][rock_j] = 'O';
                    if rock_j != j {
                        rocks[i][j] = '.';
                    }
                    rock_j = rock_j.saturating_sub(1);
                }
                _ => {}
            }
        }
    }
}

fn load_of(rocks: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for i in 0..rocks.len() {
        for j in 0..rocks[i].len() {
            if rocks[i][j] == 'O' {
                result += rocks.len() - i;
            }
        }
    }
    result
}

fn main() {
    let mut rocks = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| {
            line.chars().collect_vec()
        })
        .collect_vec();

    //rotate_cycle(&mut rocks);
    let mut cycles = vec![rocks.clone()];
    let mut cycle_position = None;

    while cycle_position == None {
        rotate_cycle(&mut rocks);
        cycle_position = cycles.iter().enumerate().find(|(_, c)| **c == rocks).map(|(i, _)| i);

        if cycle_position == None {
            cycles.push(rocks.clone());
        }
    }
    let cycle_position = cycle_position.unwrap();

    let rocks = &cycles[cycle_position + ((1000000000 - cycle_position) % (cycles.len() - cycle_position))];
    println!("{}", load_of(rocks));
}
