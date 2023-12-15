use std::io::stdin;

fn hash(s: &str) -> usize {
    let mut hash = 0;
    for c in s.chars() {
        hash = ((hash + c as usize) * 17) % 256;
    }
    hash
}

fn main() -> Result<(), ()> {
    let mut boxes = vec![Vec::<(String, usize)>::new(); 256];

    stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .next()
        .ok_or(())?
        .split(',')
        .for_each(|op| {
            let elements = op.split(['-', '=']).collect::<Vec<_>>();
            let box_nr = hash(elements[0]);

            match op.chars().filter(|c| *c == '-' || *c == '=').next() {
                Some('=') => {
                    let focal = elements[1].parse::<usize>().unwrap();

                    if boxes[box_nr]
                        .iter_mut()
                        .map(|b| {
                            if b.0 == elements[0] {
                                b.1 = focal;
                                false
                            } else {
                                true
                            }
                        })
                        .all(|a| a)
                    {
                        boxes[box_nr].push((elements[0].to_owned(), focal));
                    }
                }
                Some('-') => boxes[box_nr].retain(|el| el.0 != elements[0]),
                _ => {}
            }
        });

    let result: usize = boxes
        .iter()
        .enumerate()
        .map(|(b_i, b)| {
            b.iter()
                .enumerate()
                .map(move |(l_i, (_, l))| (b_i + 1) * (l_i + 1) * l)
        })
        .flatten()
        .sum();
    println!("{result}");
    Ok(())
}
