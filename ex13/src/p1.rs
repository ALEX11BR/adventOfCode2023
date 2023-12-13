use std::io::stdin;
use itertools::Itertools;

fn line_eq(array: &Vec<Vec<char>>, i1: usize, i2: usize) -> bool {
    for j in 0..array[0].len() {
        if array[i1][j] != array[i2][j] {
            return false;
        }
    }
    true
}

fn column_eq(array: &Vec<Vec<char>>, j1: usize, j2: usize) -> bool {
    for i in 0..array.len() {
        if array[i][j1] != array[i][j2] {
            return false;
        }
    }
    true
}

fn main() {
    let groups = &stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .group_by(|line| line.is_empty());

    let result = groups
        .into_iter()
        .filter_map(|(empty, data)| {
            if empty {
                None
            } else {
                Some(data.collect_vec())
            }
        })
        .map(|land| {
            let land = land.into_iter()
                .map(|line| {
                    line.chars().collect_vec()
                })
                .collect_vec();

            let mut sym_i = None;
            let mut sym_j = None;

            'for_i: for i in 0..(land.len() - 1) {
                if i > land.len() / 2 && sym_i != None {
                    break 'for_i;
                }

                if line_eq(&land, i, i+1) {
                    let mut j = 1;
                    while i >= j && i + 1 + j < land.len() {
                        if !line_eq(&land, i - j, i + 1 + j) {
                            continue 'for_i;
                        }
                        j += 1;
                    }

                    sym_i = Some(i + 1);
                }
            }

            'for_j: for j in 0..(land[0].len() - 1) {
                if j > land.len() / 2 && sym_j != None {
                    break 'for_j;
                }
                
                if column_eq(&land, j, j+1) {
                    let mut i = 1;
                    while j >= i && i + 1 + j < land[0].len() {
                        if !column_eq(&land, j - i, i + 1 + j) {
                            continue 'for_j;
                        }
                        i += 1;
                    }

                    sym_j = Some(j + 1);
                }
            }

            sym_i.unwrap_or(0) * 100 + sym_j.unwrap_or(0)
        })
        .sum::<usize>();

    println!("{result}");
}
