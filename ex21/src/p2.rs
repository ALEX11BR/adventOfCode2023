// This isn't supposed to work on the example input
use std::{collections::VecDeque, error::Error, io::stdin};

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
            .filter(|n| distances[n.0 as usize][n.1 as usize] == -1)
            .for_each(|n| queue.push_back((n, moves + 1)));
    }

    distances
}

fn count_with_steps(distance: &Vec<Vec<i64>>, steps_left: i64) -> usize {
    distance
        .iter()
        .map(|line| {
            line.iter()
                .filter(|&&x| x > -1 && x % 2 == steps_left % 2 && x <= steps_left)
        })
        .flatten()
        .count()
}

fn gauss(n: usize) -> usize {
    (n * (n + 1)) / 2
}

fn main() -> Result<(), Box<dyn Error>> {
    let map = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let starting = map
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            if let Some(j) =
                line.iter()
                    .enumerate()
                    .find_map(|(j, &c)| if c == 'S' { Some(j) } else { None })
            {
                Some((i as i64, j as i64))
            } else {
                None
            }
        })
        .ok_or("")?;

    let distance = lee(&map, starting);

    let distance_up = lee(&map, (0, starting.1));
    let distance_down = lee(&map, (map.len() as i64 - 1, starting.1));
    let distance_left = lee(&map, (starting.0, 0));
    let distance_right = lee(&map, (starting.0, map[0].len() as i64 - 1));

    let distance_up_left = lee(&map, (0, 0));
    let distance_down_left = lee(&map, (map.len() as i64 - 1, 0));
    let distance_up_right = lee(&map, (0, map[0].len() as i64 - 1));
    let distance_down_right = lee(&map, (map.len() as i64 - 1, map[0].len() as i64 - 1));

    let steps = 26501365;
    let diagonal_patterns = steps / map.len();

    let result = count_with_steps(&distance_up, map.len() as i64 - 1)
        + count_with_steps(&distance_down, map.len() as i64 - 1)
        + count_with_steps(&distance_left, map.len() as i64 - 1)
        + count_with_steps(&distance_right, map.len() as i64 - 1)
        + diagonal_patterns * count_with_steps(&distance_up_left, map.len() as i64 / 2 - 1)
        + diagonal_patterns * count_with_steps(&distance_up_right, map.len() as i64 / 2 - 1)
        + diagonal_patterns * count_with_steps(&distance_down_left, map.len() as i64 / 2 - 1)
        + diagonal_patterns * count_with_steps(&distance_down_right, map.len() as i64 / 2 - 1)
        + (diagonal_patterns - 1) * count_with_steps(&distance_up_left, (map.len() as i64 / 2) * 3)
        + (diagonal_patterns - 1)
            * count_with_steps(&distance_up_right, (map.len() as i64 / 2) * 3)
        + (diagonal_patterns - 1)
            * count_with_steps(&distance_down_left, (map.len() as i64 / 2) * 3)
        + (diagonal_patterns - 1)
            * count_with_steps(&distance_down_right, (map.len() as i64 / 2) * 3)
        + (1 + 4 * 2 * gauss(diagonal_patterns / 2 - 1))
            * count_with_steps(&distance, steps as i64)
        + (4 * (gauss(diagonal_patterns / 2) + gauss(diagonal_patterns / 2 - 1)))
            * count_with_steps(&distance, steps as i64 + 1);
    println!("{result}");

    Ok(())
}
