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
    fn is_not_full(self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
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
            let game_id = game_fields[0]
                .split(' ')
                .nth(1)
                .and_then(|n| i32::from_str_radix(n, 10).ok());

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
                .all(|b: BallsNumber| b.is_not_full())
                .then_some(0)
                .and_then(|_| game_id)
        })
        .sum();

    println!("{result}");
}
