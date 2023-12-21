use std::{io::stdin, error::Error, collections::VecDeque};

use itertools::Itertools;

fn neighbors_of<'a>(
    point: &'a (i64, i64),
    map: &'a Vec<Vec<char>>,
) -> impl Iterator<Item = (i64, i64)> + 'a {
    (-2..=-1)
        .chain(1..=2)
        .map(|i| (point.0 + i / 2, point.1 + i % 2))
        .filter(|m| {
            m.0 >= 0
                && m.1 >= 0
                && m.0 < map.len() as i64
                && m.1 < map[m.0 as usize].len() as i64
                && map[m.0 as usize][m.1 as usize] != '#'
        })
}

fn lee(map: &Vec<Vec<char>>, starting: (i64, i64)) -> Vec<Vec<i64>> {
    let mut distances = vec![vec![-1; map[0].len()]; map.len()];

    let mut queue = VecDeque::from([(starting, 0)]);

    while let Some((point, moves)) = queue.pop_front() {
        if distances[point.0 as usize][point.1 as usize] > -1 {
            continue;
        }

        distances[point.0 as usize][point.1 as usize] = moves;

        neighbors_of(&point, map)
            .filter(|n| {
                distances[n.0 as usize][n.1 as usize] == -1
            })
            .for_each(|n| {
                queue.push_back((n, moves + 1))
            });
    }

    distances
}

fn main() -> Result<(), Box<dyn Error>> {
    let map = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| {
            line.chars().collect_vec()
        })
        .collect_vec();

    let starting = map
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            if let Some(j) = line.iter().enumerate().find_map(|(j, &c)| {
                if c == 'S' {
                    Some(j)
                } else {
                    None
                }
            })
            {
                Some((i as i64, j as i64))
            } else {
                None
            }
        })
        .ok_or("")?;

    let distance = lee(&map, starting);

    let times = 64;

    let result = distance
        .iter()
        .map(|line| {
            line
                .iter()
                .filter(|&&x| {
                    x > -1 && x % 2 == times % 2 && x <= times as i64
                })
        })
        .flatten()
        .count();
    println!("{result}");

    Ok(())
}
