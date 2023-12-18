use std::{io::stdin, error::Error};

use itertools::Itertools;

fn move_by(point: (i64, i64), direction: (i64, i64)) -> (i64, i64) {
    (point.0 + direction.0, point.1 + direction.1)
}

fn direction_with_length(direction: (i64, i64), length: u64) -> (i64, i64) {
    (direction.0 * length as i64, direction.1 * length as i64)
}

fn parse_direction(direction: u64) -> (i64, i64) {
    match direction {
        0 => (0, 1),
        2 => (0, -1),
        1 => (1, 0),
        3 => (-1, 0),
        _ => (0, 0)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let line_regex = regex::Regex::new(r#"(.) ([0-9]+) \(\#([0-9a-f]+)\)"#)?;

    let mut current_position = (0, 0);

    let edges = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| {
            let caps = line_regex.captures(&line).unwrap();

            let color = u64::from_str_radix(&caps[3], 16).unwrap_or(0);

            (parse_direction(color & 0xf), color >> 4, color)
        })
        .map(|(direction, length, color)| {
            let new_position = move_by(current_position, direction_with_length(direction, length));
            let old_position = current_position;

            current_position = new_position;
            (old_position, new_position, color)
        })
        .collect_vec();

    let mut area = 0;
    let mut perimeter = 0;

    for edge in edges {
        perimeter += (edge.0.0 - edge.1.0).abs() + (edge.0.1 - edge.1.1).abs();
        area += edge.0.0 * edge.1.1 - edge.0.1 * edge.1.0;
    }
    area = area.abs() / 2 + perimeter / 2 + 1;

    println!("{area}");

    Ok(())
}
