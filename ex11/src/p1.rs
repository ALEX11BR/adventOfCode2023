use std::io::stdin;

fn main() {
    let mut width = 0;
    let mut height = 0;

    let galaxies = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .enumerate()
        .map(|(i, line)| {
            height = i + 1;
            line.chars()
                .enumerate()
                .filter_map(|(j, c)| {
                    width = j + 1;
                    match c {
                        '#' => Some((i, j)),
                        _ => None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut tiles_i = vec![0usize; height];
    let mut tiles_j = vec![0usize; width];

    for i in 1..height {
        tiles_i[i] = tiles_i[i - 1] + 1;
        
        if galaxies.iter().all(|g| g.0 != i) {
            tiles_i[i] += 1;
        }
    }

    for j in 1..height {
        tiles_j[j] = tiles_j[j - 1] + 1;

        if galaxies.iter().all(|g| g.1 != j) {
            tiles_j[j] += 1;
        }
    }

    let mut result = 0;

    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let galaxy_a = &galaxies[i];
            let galaxy_b = &galaxies[j];
            let distance = tiles_i[galaxy_a.0].abs_diff(tiles_i[galaxy_b.0]) + tiles_j[galaxy_a.1].abs_diff(tiles_j[galaxy_b.1]);
            result += distance;
        }
    }
    println!("{result}");
}
