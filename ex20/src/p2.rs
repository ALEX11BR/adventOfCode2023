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
    Conjunction(Vec<String>, HashMap<String, (bool, i64)>),
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
                    let _ = on_status.insert(module.0.to_owned(), (false, -1));
                }
            }
        }
    });

    let module_of_rx = modules
        .iter()
        .find(|(_, module)| {
            matches!(module, Module::Conjunction(dests, _) if dests.iter().any(|d| d == "rx"))
        })
        .map(|(id, _)| id.to_owned())
        .unwrap_or("".to_owned());

    let mut button_presses = 0;

    loop {
        let mut event_queue = VecDeque::new();
        event_queue.push_back(("broadcaster".to_owned(), "button".to_owned(), false));
        button_presses += 1;

        while let Some((current, previous, is_high)) = event_queue.pop_front() {
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
                    on_status
                        .get_mut(&previous)
                        .map(|v| {
                            v.0 = is_high;

                            if is_high && v.1 == -1 {
                                v.1 = button_presses;
                            }
                        });

                    let to_send = on_status.iter().any(|s| !s.1.0);
                    for item in linked {
                        event_queue.push_back((item.to_owned(), current.clone(), to_send));
                    }
                }
            }
        }

        let Some(Module::Conjunction(_, dependents)) = modules.get(&module_of_rx) else {
            return Err("???".into());
        };

        if dependents.values().all(|d| d.1 > -1) {
            let result = dependents
                .values()
                .map(|d| d.1)
                .fold(1, num::integer::lcm);

            println!("{result}");
            return Ok(());
        }
    }
}
