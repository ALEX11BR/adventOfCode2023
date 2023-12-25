// Uses `z3`: it can be tricky to compile.
use std::{error::Error, io::stdin};

use itertools::Itertools;
use z3::ast::Ast;

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

    let constraints = 20;

    let z3_cfg = z3::Config::new();
    let z3_ctx = z3::Context::new(&z3_cfg);
    let z3_solver = z3::Solver::new(&z3_ctx);

    let rock_x0 = z3::ast::Int::new_const(&z3_ctx, "rock_x0");
    let rock_x1 = z3::ast::Int::new_const(&z3_ctx, "rock_x1");
    let rock_y0 = z3::ast::Int::new_const(&z3_ctx, "rock_y0");
    let rock_y1 = z3::ast::Int::new_const(&z3_ctx, "rock_y1");
    let rock_z0 = z3::ast::Int::new_const(&z3_ctx, "rock_z0");
    let rock_z1 = z3::ast::Int::new_const(&z3_ctx, "rock_z1");
    let z3_zero = z3::ast::Int::from_i64(&z3_ctx, 0);

    for (i, particle) in particles.iter().enumerate().take(constraints) {
        let t = z3::ast::Int::new_const(&z3_ctx, format!("t{}", i));

        let particle_x0 = z3::ast::Int::from_i64(&z3_ctx, particle.0.0);
        let particle_x1 = z3::ast::Int::from_i64(&z3_ctx, particle.1.0);
        let particle_y0 = z3::ast::Int::from_i64(&z3_ctx, particle.0.1);
        let particle_y1 = z3::ast::Int::from_i64(&z3_ctx, particle.1.1);
        let particle_z0 = z3::ast::Int::from_i64(&z3_ctx, particle.0.2);
        let particle_z1 = z3::ast::Int::from_i64(&z3_ctx, particle.1.2);

        z3_solver.assert(&t.gt(&z3_zero));
        z3_solver.assert(&(&rock_x0 + &rock_x1 * &t)._eq(&(&particle_x0 + &particle_x1 * &t)));
        z3_solver.assert(&(&rock_y0 + &rock_y1 * &t)._eq(&(&particle_y0 + &particle_y1 * &t)));
        z3_solver.assert(&(&rock_z0 + &rock_z1 * &t)._eq(&(&particle_z0 + &particle_z1 * &t)));
    }

    z3_solver.check();
    let result = z3_solver.get_model().and_then(|model| {
        model.eval(&(rock_x0 + rock_y0 + rock_z0), true)
    }).ok_or("")?;

    println!("{result}");

    Ok(())
}
