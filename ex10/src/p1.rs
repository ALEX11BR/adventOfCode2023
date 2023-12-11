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

fn calc_length(space: &Vec<Vec<Vec<(i64, i64)>>>, start_from: &(i64, i64)) -> i64 {
    let mut length = 1;
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
        length += 1;
        let new_previous = current;
        current = *space[new_previous.0 as usize][new_previous.1 as usize]
            .iter()
            .filter(|m| **m != previous)
            .next()
            .unwrap();
        previous = new_previous;
    }

    length / 2
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

    let result = calc_length(&space, &start);
    println!("{result}");
}
