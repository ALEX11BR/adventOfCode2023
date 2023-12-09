use std::{collections::HashMap, error::Error, io::stdin};

fn solve_for_node(
    start_node: &str,
    graph: &HashMap<String, [String; 2]>,
    moves: &Vec<usize>,
) -> i64 {
    let mut current_node = start_node.to_owned();
    let mut result = 0;
    for mov in moves.iter().cycle() {
        if current_node.ends_with('Z') {
            break;
        }

        result += 1;
        current_node = graph[&current_node][*mov].clone();
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut pattern = String::new();
    stdin().read_line(&mut pattern)?;

    let moves = pattern
        .chars()
        .filter_map(|c| match c {
            'L' => Some(0),
            'R' => Some(1),
            _ => None,
        })
        .collect::<Vec<usize>>();

    let line_regex = regex::Regex::new(r#"^([A-Z]*) \= \(([A-Z]*)\, ([A-Z]*)\)$"#)?;

    let graph = stdin()
        .lines()
        .skip(1)
        .filter_map(|result_line| result_line.ok())
        .map(|line| {
            let caps = line_regex.captures(&line).unwrap();

            (caps[1].to_owned(), [caps[2].to_owned(), caps[3].to_owned()])
        })
        .collect::<HashMap<_, _>>();

    let result = graph
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| solve_for_node(k, &graph, &moves))
        .fold(1, num::integer::lcm);

    println!("{result}");

    Ok(())
}
