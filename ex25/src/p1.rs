use std::{io::stdin, collections::{HashMap, HashSet}};

use itertools::Itertools;
use rand::seq::IteratorRandom;

fn graph_add(graph: &mut HashMap<String, Vec<String>>, source: &str, destination: &str) {
    if !graph.contains_key(source) {
        graph.insert(source.to_owned(), Vec::new());
    }
    if !graph.contains_key(destination) {
        graph.insert(destination.to_owned(), Vec::new());
    }

    graph.get_mut(source).map(|r| r.push(destination.to_owned()));
    graph.get_mut(destination).map(|r| r.push(source.to_owned()));
}

fn karger(graph: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut cut_graph = graph.clone();

    while cut_graph.len() > 2 {
        let Some(node1) = cut_graph.keys().choose(&mut rand::thread_rng()).map(|r| r.clone()) else {
            break;
        };
        let Some(node2) = cut_graph[&node1].iter().choose(&mut rand::thread_rng()).map(|r| r.clone()) else {
            break;
        };

        let new_node = format!("{}{}", node1, node2);
        let new_node_neighbors = cut_graph[&node1].iter().chain(cut_graph[&node2].iter()).filter(|&s| s != &node1 && s != &node2).map(|s| s.clone()).collect_vec();

        for neighbor in new_node_neighbors.iter().unique() {
            cut_graph.get_mut(neighbor).map(|neighbors| neighbors.iter_mut().filter(|n| *n == &node1 || *n == &node2).for_each(|n| {
                *n = new_node.clone();
            }));
        }
        cut_graph.remove(&node1);
        cut_graph.remove(&node2);
        cut_graph.insert(new_node, new_node_neighbors);
    }

    cut_graph
}

fn main() {
    let mut nodes = HashSet::new();
    let mut graph = HashMap::new();

    stdin()
        .lines()
        .filter_map(|result_line| result_line.ok())
        .for_each(|line| {
            let Some((source, destinations)) = line.split(": ").collect_tuple() else {
                return;
            };

            nodes.insert(source.to_owned());
            for destination in destinations.split_ascii_whitespace() {
                nodes.insert(destination.to_owned());
                graph_add(&mut graph, source, destination)
            }
        });

    let result = loop {
        let cut = karger(&graph);

        let cut_edges = cut.values().next().map(|n| n.len()).unwrap_or(4);

        if cut_edges == 3 {
            break cut.keys().map(|k| k.chars().count() / 3).product::<usize>();
        }
    };

    println!("{result}");
}
