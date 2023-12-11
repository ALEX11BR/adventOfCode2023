// Late entry: at about 08:20 UTC+2, 1h 20min after day 11 started.

use std::io::stdin;

fn neighbors_of<'a>(
    point: &'a (i64, i64),
    space: &'a Vec<Vec<Vec<(i64, i64)>>>,
) -> impl Iterator<Item = (i64, i64)> + 'a {
    (-2..=-1)
        .chain(1..=2)
        .map(|i| (point.0 + i / 2, point.1 + i % 2))
        .filter(|m| {
            m.0 >= 0
                && m.1 >= 0
                && m.0 < space.len() as i64
                && m.1 < space[m.0 as usize].len() as i64
                && space[m.0 as usize][m.1 as usize].len() > 0
        })
}

fn calc_tiles(space: &Vec<Vec<Vec<(i64, i64)>>>, start_from: &(i64, i64)) -> Vec<(i64, i64)> {
    let mut tiles = vec![*start_from];

    let mut current = neighbors_of(start_from, space)
        .filter(|m| {
            space[m.0 as usize][m.1 as usize]
                .iter()
                .any(|n| n == start_from)
        })
        .next()
        .unwrap();
    let mut previous = *start_from;

    while current != *start_from {
        tiles.push(current);
        let new_previous = current;
        current = *space[new_previous.0 as usize][new_previous.1 as usize]
            .iter()
            .filter(|m| **m != previous)
            .next()
            .unwrap();
        previous = new_previous;
    }

    tiles
}

fn is_point_in(tiles: &Vec<(i64, i64)>, point: (i64, i64)) -> bool {
    let mut result = false;

    for i in 0..tiles.len() {
        let tile_a = tiles[i];
        let tile_b = tiles[if i == tiles.len() - 1 { 0 } else { i + 1 }];

        if tile_a == point {
            return false;
        }
        if (tile_a.1 > point.1) != (tile_b.1 > point.1) {
            let slope = (point.0 - tile_a.0) * (tile_b.1 - tile_a.1)
                - (point.1 - tile_a.1) * (tile_b.0 - tile_a.0);
            if slope == 0 {
                return false;
            }
            if (slope < 0) != (tile_b.1 < tile_a.1) {
                result = !result;
            }
        }
    }

    result
}

fn main() {
    let mut start = (0, 0);
    let space = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .enumerate()
        .map(|(i, line)| {
            let i = i as i64;
            line.chars()
                .enumerate()
                .filter_map(|(j, c)| {
                    let j = j as i64;
                    match c {
                        '|' => Some(vec![(i + 1, j), (i - 1, j)]),
                        '-' => Some(vec![(i, j + 1), (i, j - 1)]),
                        'F' => Some(vec![(i + 1, j), (i, j + 1)]),
                        'L' => Some(vec![(i - 1, j), (i, j + 1)]),
                        'J' => Some(vec![(i - 1, j), (i, j - 1)]),
                        '7' => Some(vec![(i + 1, j), (i, j - 1)]),
                        '.' => Some(vec![]),
                        'S' => {
                            start = (i, j);
                            Some(vec![(i, j)])
                        }
                        _ => None,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let tiles = calc_tiles(&space, &start);

    let mut result = 0;

    for i in 0..space.len() {
        for j in 0..space[i].len() {
            if is_point_in(&tiles, (i as i64, j as i64)) {
                result += 1;
            }
        }
    }
    println!("{result}");
}
