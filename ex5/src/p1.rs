use std::{collections::HashMap, io::stdin};

#[derive(Debug, Default)]
struct RangeMap {
    destination_start: i64,
    source_start: i64,
    length: i64,
}
impl RangeMap {
    fn new(destination_start: i64, source_start: i64, length: i64) -> Self {
        Self {
            destination_start,
            source_start,
            length,
        }
    }
}
fn find_mapping(ranges: &Vec<RangeMap>, source: i64) -> i64 {
    ranges
        .iter()
        .filter(|r| r.source_start <= source && r.length + r.source_start > source)
        .map(|r| r.destination_start + source - r.source_start)
        .nth(0)
        .unwrap_or(source)
}

fn main() {
    let mut seeds = vec![];
    let mut mappings = HashMap::<(String, String), Vec<RangeMap>>::new();

    let mut source = "seed".to_owned();
    let mut destination = "location".to_owned();

    for line in stdin().lines().filter_map(|result_line| result_line.ok()) {
        match line.split(':').nth(0) {
            Some("seeds") => {
                line.split(':').nth(1).map(|numbers| {
                    seeds = numbers
                        .split_ascii_whitespace()
                        .filter_map(|nr| nr.parse::<i64>().ok())
                        .collect()
                });
            }
            Some(s) if s.ends_with(" map") => {
                let field_map = s
                    .split_ascii_whitespace()
                    .nth(0)
                    .unwrap_or_default()
                    .split('-')
                    .collect::<Vec<_>>();

                source = field_map[0].to_owned();
                destination = field_map[2].to_owned();
                mappings.insert((source.clone(), destination.clone()), Vec::new());
            }
            Some(s) if s != "" => {
                let numbers = s
                    .split_ascii_whitespace()
                    .filter_map(|num| num.parse::<i64>().ok())
                    .collect::<Vec<_>>();
                mappings
                    .get_mut(&(source.clone(), destination.clone()))
                    .unwrap()
                    .push(RangeMap::new(numbers[0], numbers[1], numbers[2]));
            }
            _ => {}
        }
    }

    let result = seeds
        .iter()
        .map(|seed| {
            let mut source = "seed".to_owned();
            let mut destination;
            let mut result = *seed;

            while let Some(key) = mappings.keys().find(|key| key.0 == source.clone()) {
                destination = key.1.clone();

                result = find_mapping(mappings.get(key).unwrap(), result);

                source = destination;
            }

            result
        })
        .min()
        .unwrap_or(0);

    println!("{result}");
}
