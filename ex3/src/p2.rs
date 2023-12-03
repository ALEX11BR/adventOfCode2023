use std::io::stdin;

fn neighbors_a_symbol(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let i_min = if i != 0 {i-1} else {0};
    let i_max = if i < chars.len() - 1 {i+1} else {i};
    let j_min = if j != 0 {j-1} else {0};
    let j_max = if j < chars[i].len() - 1 {j+1} else {j};

    for ii in i_min..=i_max {
        for jj in j_min..=j_max {
            if !chars[ii][jj].is_ascii_alphanumeric() && chars[ii][jj] != '.' {
                return true;
            }
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
    for i in 0..chars.len() {
        let mut current_number = 0;
        let mut neighbors_symbols = false;

        for j in 0..chars[i].len() {
            if let Some(digit) = chars[i][j].to_digit(10) {
                current_number = current_number * 10 + digit;
                neighbors_symbols |= neighbors_a_symbol(&chars, i, j);
            } else {
                if neighbors_symbols {
                    result += current_number;
                }
                current_number = 0;
                neighbors_symbols = false;
            }
        }

        if neighbors_symbols {
            result += current_number;
        }
    }

    println!("{result}");
}
