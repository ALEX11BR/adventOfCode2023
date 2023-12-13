use std::io::stdin;
use itertools::Itertools;

fn line_diffs(array: &Vec<Vec<char>>, i1: usize, i2: usize) -> usize {
    let mut differences = 0;
    for j in 0..array[0].len() {
        if array[i1][j] != array[i2][j] {
            differences += 1;
        }
    }
    differences
}

fn column_diffs(array: &Vec<Vec<char>>, j1: usize, j2: usize) -> usize {
    let mut differences = 0;
    for i in 0..array.len() {
        if array[i][j1] != array[i][j2] {
            differences += 1;
        }
    }
    differences
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

                let mut diff = line_diffs(&land, i, i+1);

                let mut j = 1;
                while i >= j && i + 1 + j < land.len() {
                    diff += line_diffs(&land, i - j, i + 1 + j);
                    j += 1;
                }

                if diff == 1 {
                    sym_i = Some(i + 1);
                }
            }

            'for_j: for j in 0..(land[0].len() - 1) {
                if j > land.len() / 2 && sym_j != None {
                    break 'for_j;
                }
                
                let mut diff = column_diffs(&land, j, j+1);
                let mut i = 1;
                while j >= i && i + 1 + j < land[0].len() {
                    diff += column_diffs(&land, j - i, i + 1 + j);
                    i += 1;
                }

                if diff == 1 {
                    sym_j = Some(j + 1);
                }
            }

            sym_i.unwrap_or(0) * 100 + sym_j.unwrap_or(0)
        })
        .sum::<usize>();

    println!("{result}");
}
