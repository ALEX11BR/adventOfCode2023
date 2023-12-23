use std::{error::Error, io::stdin, vec};

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

    let mut depends_on = vec![vec![]; bricks.len()];

    for i in 0..bricks.len() {
        let self_height = bricks[i].1 .2 - bricks[i].0 .2;

        depends_on[i] = bricks
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

        if depends_on[i].is_empty() {
            bricks[i].0 .2 = 1;
        } else {
            bricks[i].0 .2 = depends_on[i][0].1 .1 .2 + 1;
        }

        bricks[i].1 .2 = bricks[i].0 .2 + self_height;
    }

    let mut result = 0;

    for i in (0..bricks.len()).rev() {
        let mut brick_gone = vec![false; bricks.len()];
        brick_gone[i] = true;

        while let Some(gone_brick) = (0..bricks.len())
            .filter(|&x| {
                !brick_gone[x]
                    && !depends_on[x].is_empty()
                    && depends_on[x].iter().all(|(b, _)| brick_gone[*b])
            })
            .next()
        {
            brick_gone[gone_brick] = true;
        }

        result += brick_gone.iter().filter(|&&x| x).count() - 1;
    }

    println!("{result}");

    Ok(())
}
