use std::io::stdin;

fn hash(s: &str) -> u32 {
    let mut hash = 0;
    for c in s.chars() {
        hash = ((hash + c as u32) * 17) % 256;
    }
    hash
}

fn main() -> Result<(), ()> {
    let result: u32 = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .next()
        .ok_or(())?
        .split(',')
        .map(hash)
        .sum();

    println!("{result}");
    Ok(())
}
