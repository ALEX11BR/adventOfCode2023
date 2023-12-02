use std::{i32, io::stdin, ops::Add};

#[derive(Debug, Clone, Copy, Default)]
struct BallsNumber {
    red: i32,
    green: i32,
    blue: i32,
}
impl BallsNumber {
    fn new_by_name(amount: i32, color: &str) -> Self {
        BallsNumber {
            red: if color == "red" { amount } else { 0 },
            green: if color == "green" { amount } else { 0 },
            blue: if color == "blue" { amount } else { 0 },
        }
    }
    fn max_values(self, other: Self) -> Self {
        BallsNumber {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}
impl Add for BallsNumber {
    type Output = BallsNumber;
    fn add(self, rhs: Self) -> Self::Output {
        BallsNumber {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

fn main() {
    let result: i32 = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .filter_map(|game_line| {
            let game_fields = game_line.split(':').collect::<Vec<_>>();

            game_fields[1]
                .split(';')
                .map(|round| {
                    round
                        .split(',')
                        .map(|ball| {
                            let ball_fields = ball.split_ascii_whitespace().collect::<Vec<_>>();
                            BallsNumber::new_by_name(
                                i32::from_str_radix(ball_fields[0], 10).unwrap_or_default(),
                                ball_fields[1],
                            )
                        })
                        .fold(Default::default(), |a, b| a + b)
                })
                .reduce(|acc: BallsNumber, e: BallsNumber| {
                    acc.max_values(e)
                })
                .map(|balls: BallsNumber| {
                    balls.red * balls.green * balls.blue
                })
        })
        .sum();

    println!("{result}");
}
