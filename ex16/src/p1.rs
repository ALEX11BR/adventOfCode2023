use std::io::stdin;

use itertools::Itertools;

fn move_by(point: (i32, i32), direction: (i32, i32)) -> (i32, i32) {
    (point.0 + direction.0, point.1 + direction.1)
}

fn walk_light(layout: &Vec<Vec<char>>, energized: &mut Vec<Vec<u8>>, start: (i32, i32), direction: (i32, i32)) {
    if start.0 < 0 || start.0 >= layout.len() as i32 || start.1 < 0 || start.1 >= layout[0].len() as i32 {
        return;
    }

    let current_char = layout[start.0 as usize][start.1 as usize];

    if direction.0 == 0 {
        if energized[start.0 as usize][start.1 as usize] & 1 == 0 {
            energized[start.0 as usize][start.1 as usize] |= 1;
        } else if current_char != '\\' && current_char != '/'{
            return;
        }
    } else {
        if energized[start.0 as usize][start.1 as usize] & 2 == 0 {
            energized[start.0 as usize][start.1 as usize] |= 2;
        } else if current_char != '\\' && current_char != '/' {
            return;
        }
    }

    let new_point = move_by(start, direction);
    match layout[start.0 as usize][start.1 as usize] {
        '.' => walk_light(layout, energized, new_point, direction),
        '|' => {
            if direction.1 == 0 {
                walk_light(layout, energized, new_point, direction);
            } else {
                walk_light(layout, energized, move_by(start, (1, 0)), (1, 0));
                walk_light(layout, energized, move_by(start, (-1, 0)), (-1, 0));
            }
        }
        '-' => {
            if direction.0 == 0 {
                walk_light(layout, energized, new_point, direction);
            } else {
                walk_light(layout, energized, move_by(start, (0, 1)), (0, 1));
                walk_light(layout, energized, move_by(start, (0, -1)), (0, -1));
            }
        }
        '/' => {
            let new_direction = if direction.0 == 0 {
                (-direction.1, 0)
            } else {
                (0, -direction.0)
            };
            let new_point = move_by(start, new_direction);
            walk_light(layout, energized, new_point, new_direction);
        }
        '\\' => {
            let new_direction = if direction.0 == 0 {
                (direction.1, 0)
            } else {
                (0, direction.0)
            };
            let new_point = move_by(start, new_direction);
            walk_light(layout, energized, new_point, new_direction);
        }
        _ => {}
    }
}
fn main() {
    let layout = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| {
            line.chars().collect_vec()
        })
        .collect_vec();

    let mut energized = vec![vec![0; layout[0].len()]; layout.len()];
    walk_light(&layout, &mut energized, (0, 0), (0, 1));

    let result: u32 = energized.iter().map(|l| l.iter()).flatten().map(|e| if *e > 0 {1} else {0}).sum();

    println!("{result}");
}
