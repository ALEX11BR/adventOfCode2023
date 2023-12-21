use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    io::stdin,
};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    Broadcaster(Vec<String>),
    FlipFlop(Vec<String>, bool),
    Conjunction(Vec<String>, HashMap<String, bool>),
}
impl Module {
    fn get_linked(&self) -> &[String] {
        match self {
            Module::Broadcaster(linked) => linked,
            Module::FlipFlop(linked, _) => linked,
            Module::Conjunction(linked, _) => linked,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let line_regex = regex::Regex::new(r#"(.)([a-z]*) -> (.*)"#)?;

    let mut modules = stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .filter_map(|line| {
            let Some(caps) = line_regex.captures(&line) else {
                return None;
            };

            let linked = caps[3].split(", ").map(|s| s.to_owned()).collect_vec();

            match caps[1].chars().next() {
                Some('b') => Some(("broadcaster".to_owned(), Module::Broadcaster(linked))),
                Some('%') => Some((caps[2].to_owned(), Module::FlipFlop(linked, false))),
                Some('&') => Some((
                    caps[2].to_owned(),
                    Module::Conjunction(linked, HashMap::new()),
                )),
                _ => None,
            }
        })
        .collect::<HashMap<_, _>>();
    let modules_clone = modules.clone();
    modules.iter_mut().for_each(|v| {
        if let Module::Conjunction(_, on_status) = v.1 {
            for module in modules_clone.iter() {
                if module.1.get_linked().iter().any(|l| l == v.0) {
                    let _ = on_status.insert(module.0.to_owned(), false);
                }
            }
        }
    });

    let mut lows = 0i64;
    let mut highs = 0i64;

    for _ in 0..1000 {
        let mut event_queue = VecDeque::new();
        event_queue.push_back(("broadcaster".to_owned(), "button".to_owned(), false));

        while let Some((current, previous, is_high)) = event_queue.pop_front() {
            if is_high {
                highs += 1;
            } else {
                lows += 1;
            }

            let Some(handler) = modules.get_mut(&current) else {
                continue;
            };
            match handler {
                Module::Broadcaster(linked) => {
                    for item in linked {
                        event_queue.push_back((item.to_owned(), current.clone(), false));
                    }
                }
                Module::FlipFlop(linked, on) => {
                    if !is_high {
                        let to_send = !*on;
                        *on = !*on;

                        for item in linked {
                            event_queue.push_back((item.to_owned(), current.clone(), to_send));
                        }
                    }
                }
                Module::Conjunction(linked, on_status) => {
                    on_status.get_mut(&previous).map(|v| *v = is_high);

                    let to_send = on_status.iter().any(|s| !s.1);
                    for item in linked {
                        event_queue.push_back((item.to_owned(), current.clone(), to_send));
                    }
                }
            }
        }
    }

    let result = lows * highs;

    println!("{result}");

    Ok(())
}
