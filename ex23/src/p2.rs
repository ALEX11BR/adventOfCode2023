use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    io::stdin,
    iter::{empty, once},
};

use itertools::Itertools;

fn point_difference(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 - b.0, a.1 - b.1)
}

fn all_neighbors_of<'a>(
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

fn neighbors_of<'a>(
    point: &'a (i64, i64),
    map: &'a Vec<Vec<char>>,
) -> Box<dyn Iterator<Item = (i64, i64)> + 'a> {
    match map[point.0 as usize][point.1 as usize] {
        '.' => Box::new(all_neighbors_of(point, map).filter(|&m| {
            (map[m.0 as usize][m.1 as usize] != '<' || point_difference(m, *point) != (0, 1))
                && (map[m.0 as usize][m.1 as usize] != '>'
                    || point_difference(m, *point) != (0, -1))
                && (map[m.0 as usize][m.1 as usize] != '^' || point_difference(m, *point) != (1, 0))
                && (map[m.0 as usize][m.1 as usize] != 'v'
                    || point_difference(m, *point) != (-1, 0))
        })),
        '>' => Box::new(once((point.0, point.1 + 1))),
        '<' => Box::new(once((point.0, point.1 - 1))),
        'v' => Box::new(once((point.0 + 1, point.1))),
        '^' => Box::new(once((point.0 - 1, point.1))),
        _ => Box::new(empty()),
    }
}

fn traverse_simplify(
    map: &Vec<Vec<char>>,
    start_point: (i64, i64),
    end_point: (i64, i64),
) -> Vec<Vec<i64>> {
    let mut min_distance = vec![vec![i64::MIN / 2; map[0].len()]; map.len()];

    let mut simplified = HashMap::from([(start_point, vec![]), (end_point, vec![])]);
    let mut bindings = HashMap::from([(start_point, 1), (end_point, 0)]);
    let mut bindings_count = 2;

    let mut queue = VecDeque::from([(start_point, 0, start_point)]);

    while let Some((point, steps, mut root)) = queue.pop_front() {
        if all_neighbors_of(&point, map).count() > 2 || point == end_point {
            if simplified.get(&point) == None {
                simplified.insert(point, vec![]);
                bindings.insert(point, bindings_count);
                bindings_count += 1;
            }

            simplified.get_mut(&root).map(|neighbors| {
                neighbors.push((
                    point,
                    steps - min_distance[root.0 as usize][root.1 as usize],
                ))
            });
            root = point;
        }

        if min_distance[point.0 as usize][point.1 as usize] > -1 {
            continue;
        }

        min_distance[point.0 as usize][point.1 as usize] = steps;

        neighbors_of(&point, map).for_each(|n| queue.push_back((n, steps + 1, root)));
    }

    let mut graph_array = vec![vec![i64::MIN / 2; bindings_count]; bindings_count];
    for i in 0..graph_array.len() {
        graph_array[i][i] = 0;
    }

    for (source, destinations) in simplified {
        for (destination, length) in destinations {
            graph_array[bindings[&source]][bindings[&destination]] = length;
            graph_array[bindings[&destination]][bindings[&source]] = length;
        }
    }

    graph_array
}

fn backtrack_max_road(graph: &Vec<Vec<i64>>) -> i64 {
    let mut used = vec![false; graph.len()];
    used[1] = true;
    let mut road = vec![1];

    fn backtrack(
        length: i64,
        graph: &Vec<Vec<i64>>,
        used: &mut Vec<bool>,
        road: &mut Vec<usize>,
    ) -> i64 {
        let last = *road.last().unwrap_or(&1);
        if last == 0 {
            return length;
        }

        let mut maximum = i64::MIN;

        for i in 0..graph.len() {
            if !used[i] && graph[last][i] > 0 {
                used[i] = true;
                road.push(i);

                let result = backtrack(length + graph[last][i], graph, used, road);

                road.pop();
                used[i] = false;

                maximum = maximum.max(result);
            }
        }

        maximum
    }

    backtrack(0, graph, &mut used, &mut road)
}

fn main() -> Result<(), Box<dyn Error>> {
    let map = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start_point = map[0]
        .iter()
        .enumerate()
        .find_map(|(j, &c)| if c == '.' { Some((0, j as i64)) } else { None })
        .ok_or("")?;
    let end_point = map[map.len() - 1]
        .iter()
        .enumerate()
        .find_map(|(j, &c)| {
            if c == '.' {
                Some((map.len() as i64 - 1, j as i64))
            } else {
                None
            }
        })
        .ok_or("")?;

    let simplified = traverse_simplify(&map, start_point, end_point);

    let result = backtrack_max_road(&simplified);
    println!("{result}");

    Ok(())
}
