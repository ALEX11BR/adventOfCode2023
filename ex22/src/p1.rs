use std::{error::Error, io::stdin};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let line_regex = regex::Regex::new("([0-9]*),([0-9]*),([0-9]*)~([0-9]*),([0-9]*),([0-9]*)")?;

    let mut bricks = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .filter_map(|line| {
            let Some(caps) = line_regex.captures(&line) else {
                return None;
            };

            Some((
                (
                    caps[1].parse::<i64>().unwrap_or(0),
                    caps[2].parse::<i64>().unwrap_or(0),
                    caps[3].parse::<i64>().unwrap_or(0),
                ),
                (
                    caps[4].parse::<i64>().unwrap_or(0),
                    caps[5].parse::<i64>().unwrap_or(0),
                    caps[6].parse::<i64>().unwrap_or(0),
                ),
            ))
        })
        .sorted_by_key(|el| el.0 .2)
        .collect_vec();

    let mut is_disintegrable = vec![true; bricks.len()];

    for i in 0..bricks.len() {
        let self_height = bricks[i].1 .2 - bricks[i].0 .2;

        let bricks_underneath = bricks
            .iter()
            .enumerate()
            .take(i)
            .filter(|(_, brick)| {
                bricks[i].0 .0 <= brick.1 .0
                    && brick.0 .0 <= bricks[i].1 .0
                    && bricks[i].0 .1 <= brick.1 .1
                    && brick.0 .1 <= bricks[i].1 .1
            })
            .map(|(j, brick)| (j, *brick))
            .max_set_by_key(|(_, brick)| brick.1 .2);

        if bricks_underneath.is_empty() {
            bricks[i].0 .2 = 1;
        } else {
            bricks[i].0 .2 = bricks_underneath[0].1 .1 .2 + 1;

            if bricks_underneath.len() == 1 {
                is_disintegrable[bricks_underneath[0].0] = false;
            }
        }

        bricks[i].1 .2 = bricks[i].0 .2 + self_height;
    }

    let result = is_disintegrable.iter().filter(|&&x| x).count();

    println!("{result}");

    Ok(())
}
