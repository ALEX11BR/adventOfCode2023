use std::{error::Error, io::stdin};

use itertools::Itertools;
use rational::Rational;

type Particle = ((i64, i64, i64), (i64, i64, i64));

fn is_in_direction(x0: Rational, x: Rational, starting_from_x0: bool) -> bool {
    if starting_from_x0 {
        x0 <= x
    } else {
        x <= x0
    }
}

fn particle_intersection(a: Particle, b: Particle) -> (Rational, Rational) {
    let x0a = Rational::new(a.0 .0, 1);
    let x1a = Rational::new(a.1 .0, 1);
    let y0a = Rational::new(a.0 .1, 1);
    let y1a = Rational::new(a.1 .1, 1);
    let x0b = Rational::new(b.0 .0, 1);
    let x1b = Rational::new(b.1 .0, 1);
    let y0b = Rational::new(b.0 .1, 1);
    let y1b = Rational::new(b.1 .1, 1);

    let slope_diff = y1a / x1a - y1b / x1b;
    if slope_diff == Rational::zero() {
        return (Rational::zero(), Rational::zero());
    }

    let x = (y0b - y0a + y1a * x0a / x1a - y1b * x0b / x1b) / slope_diff;
    let y = y0a + y1a * (x - x0a) / x1a;

    if is_in_direction(x0a, x, x1a > 0)
        && is_in_direction(x0b, x, x1b > 0)
        && is_in_direction(y0a, y, y1a > 0)
        && is_in_direction(y0b, y, y1b > 0)
    {
        (x, y)
    } else {
        (Rational::zero(), Rational::zero())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let line_regex = regex::Regex::new(
        r#"([-0-9]*), *([-0-9]*), *([-0-9]*) *@ *([-0-9]*), *([-0-9]*), *([-0-9]*)"#,
    )?;

    let particles = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| {
            let caps = line_regex.captures(&line).unwrap();

            (
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
            )
        })
        .collect_vec();

    let min_bound = Rational::new(200000000000000i64, 1);
    let max_bound = Rational::new(400000000000000i64, 1);

    let mut result = 0;

    for i in 1..particles.len() {
        for j in 0..i {
            let intersection = particle_intersection(particles[i], particles[j]);

            if intersection.0 >= min_bound
                && intersection.0 <= max_bound
                && intersection.1 >= min_bound
                && intersection.1 <= max_bound
            {
                result += 1;
            }
        }
    }

    println!("{result}");

    Ok(())
}
