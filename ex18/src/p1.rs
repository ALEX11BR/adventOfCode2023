use std::{io::stdin, error::Error};

use itertools::Itertools;

fn move_by(point: (i32, i32), direction: (i32, i32)) -> (i32, i32) {
    (point.0 + direction.0, point.1 + direction.1)
}

fn direction_with_length(direction: (i32, i32), length: u32) -> (i32, i32) {
    (direction.0 * length as i32, direction.1 * length as i32)
}

fn parse_direction(direction: &str) -> (i32, i32) {
    match direction.chars().next() {
        Some('R') => (0, 1),
        Some('L') => (0, -1),
        Some('D') => (1, 0),
        Some('U') => (-1, 0),
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

            (parse_direction(&caps[1]), u32::from_str_radix(&caps[2], 10).unwrap_or(9), u32::from_str_radix(&caps[3], 16).unwrap_or(0))
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
