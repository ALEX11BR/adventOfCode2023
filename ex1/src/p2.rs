use std::io::stdin;

fn main() {
    let result: u32 = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| {
            let digits: Vec<_> = line
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|digit| digit.to_digit(10).unwrap_or(0))
                .collect();
            digits[0] * 10 + digits[digits.len() - 1]
        })
        .sum();

    println!("{result}");
}
