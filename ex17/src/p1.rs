use std::{collections::VecDeque, io::stdin};

use itertools::Itertools;

/// the minimum cost of a square, depending on how many other squares it needs to go to
#[derive(Clone, Debug)]
struct MinimumCostInfo {
    /// to_direction[i][j] = the minimum, knowing it can move to j+1 squares to the direction of i: {above, right, bottom, left}
    to_direction: [[u32; 3]; 4],
    nowhere: u32
}
impl MinimumCostInfo {
    fn new_with_cost(cost: u32) -> Self {
        Self {
            to_direction: [[cost; 3]; 4],
            nowhere: cost
        }
    }

    fn maybe_update(&mut self, new_cost: u32, flexibility: [u32; 4]) -> bool {
        let mut result = false;

        for i in 0..4 {
            for j in 0..3 {
                if flexibility[i] as usize > j && self.to_direction[i][j] > new_cost {
                    result = true;
                    self.to_direction[i][j] = new_cost;
                }
            }
        }

        if self.nowhere > new_cost {
            self.nowhere = new_cost;
            true
        } else {
            result
        }
    }
}

fn move_by(point: (i32, i32), direction: (i32, i32)) -> (i32, i32) {
    (point.0 + direction.0, point.1 + direction.1)
}

fn in_bounds(point: (i32, i32), layout: &Vec<Vec<u32>>) -> bool {
    point.0 >= 0
        && point.1 >= 0
        && point.0 < layout.len() as i32
        && point.1 < layout[0].len() as i32
}

fn solve(layout: &Vec<Vec<u32>>) -> u32 {
    let mut heat_to = vec![vec![MinimumCostInfo::new_with_cost(u32::MAX); layout[0].len()]; layout.len()];
    heat_to[0][0] = MinimumCostInfo::new_with_cost(0);

    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0, [0, 3, 3, 0]));

    while let Some((point, cost, moves_left)) = queue.pop_front() {
        let point_above = move_by(point, (-1, 0));
        if moves_left[0] > 0 && in_bounds(point_above, layout)
        {
            let new_cost = cost + layout[point_above.0 as usize][point_above.1 as usize];
            let new_flexibility = [moves_left[0] - 1, 3, 0, 3];
            if heat_to[point_above.0 as usize][point_above.1 as usize].maybe_update(new_cost, new_flexibility) {
                queue.push_back((point_above, new_cost, new_flexibility));
            }
        }

        let point_right = move_by(point, (0, 1));
        if moves_left[1] > 0 && in_bounds(point_right, layout)
        {
            let new_cost = cost + layout[point_right.0 as usize][point_right.1 as usize];
            let new_flexibility = [3, moves_left[1] - 1, 3, 0];
            if heat_to[point_right.0 as usize][point_right.1 as usize].maybe_update(new_cost, new_flexibility) {
                queue.push_back((point_right, new_cost, new_flexibility));
            }
        }

        let point_bottom = move_by(point, (1, 0));
        if moves_left[2] > 0 && in_bounds(point_bottom, layout)
        {
            let new_cost = cost + layout[point_bottom.0 as usize][point_bottom.1 as usize];
            let new_flexibility = [0, 3, moves_left[2] - 1, 3];
            if heat_to[point_bottom.0 as usize][point_bottom.1 as usize].maybe_update(new_cost, new_flexibility) {
                queue.push_back((point_bottom, new_cost, new_flexibility));
            }
        }

        let point_left = move_by(point, (0, -1));
        if moves_left[3] > 0 && in_bounds(point_left, layout)
        {
            let new_cost = cost + layout[point_left.0 as usize][point_left.1 as usize];
            let new_flexibility = [3, 0, 3, moves_left[3] - 1];
            if heat_to[point_left.0 as usize][point_left.1 as usize].maybe_update(new_cost, new_flexibility) {
                queue.push_back((point_left, new_cost, new_flexibility));
            }
        }
    }

    heat_to[heat_to.len() - 1][heat_to[0].len() - 1].nowhere
}

fn main() {
    let layout = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(0))
                .collect_vec()
        })
        .collect_vec();

    println!("{}", solve(&layout));
}
