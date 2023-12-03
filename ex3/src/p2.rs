use std::io::stdin;

#[derive(Debug, Clone, Copy, Default)]
struct Number {
    line: usize,
    c_start: usize,
    c_end: usize,
    value: u32,
}

fn neighbors_the_number(gear: (usize, usize), number: Number) -> bool {
    if gear.0.abs_diff(number.line) > 1 {
        return false;
    }

    for k in number.c_start..=number.c_end {
        if k.abs_diff(gear.1) <= 1 {
            return true;
        }
    }
    false
}

fn main() {
    let chars = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| {
            line
                .chars()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut result = 0;
    let mut gears = Vec::<(usize, usize)>::new();
    let mut numbers = Vec::<Number>::new();

    for i in 0..chars.len() {
        let mut current_number = Number::default();
        let mut building_number = false;

        for j in 0..chars[i].len() {
            if let Some(digit) = chars[i][j].to_digit(10) {
                if !building_number {
                    building_number = true;
                    current_number = Number {
                        line: i,
                        c_start: j,
                        c_end: j,
                        value: 0
                    };
                }
                current_number.value = current_number.value * 10 + digit;
                current_number.c_end = j;
            } else {
                if building_number {
                    numbers.push(current_number);
                    building_number = false;
                }

                if chars[i][j] == '*' {
                    gears.push((i, j));
                }
            }
        }

        if building_number {
            numbers.push(current_number);
        }
    }

    for gear in gears.iter() {
        let mut number_count = 0;
        let mut gear_ratio = 1;

        for number in numbers.iter() {
            if neighbors_the_number(*gear, *number) {
                number_count += 1;
                if number_count > 2 {
                    break;
                }
                gear_ratio *= number.value;
            }
        }

        if number_count == 2 {
            result += gear_ratio;
        }
    }

    println!("{result}");
}
