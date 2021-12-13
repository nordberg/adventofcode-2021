use crate::utils::get_adjacent;
use std::cmp::{max, min};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Binary, Debug, Formatter, Write};
use std::fs::{read, File};
use std::io::{self, prelude::*, BufReader};
use std::ops::Index;
use std::usize;

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut cave_connections = HashMap::new();

    for line in reader.lines() {
        let l = line.expect("Could not read line");
        let (c1, c2) = l.split_once('-').expect("Could not parse caves");

        let c11 = String::from(c1);
        let c22 = String::from(c2);

        cave_connections
            .entry(c11.clone())
            .and_modify(|c: &mut Vec<String>| c.push(c22.clone()))
            .or_insert(vec![c22.clone()]);

        cave_connections
            .entry(c22.clone())
            .and_modify(|c| c.push(c11.clone()))
            .or_insert(vec![c11.clone()]);
    }

    let answer = calculate_entry_count("start", &cave_connections, HashMap::new());

    println!("{:?}", answer);
    Ok(())
}

fn calculate_entry_count<'a>(
    current_node: &'a str,
    connections: &'a HashMap<String, Vec<String>>,
    mut visited: HashMap<String, i32>,
) -> i64 {
    if current_node == "end" {
        return 1;
    }

    if current_node.chars().all(|c| c.is_ascii_lowercase()) {
        visited
            .entry(current_node.to_string())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    let mut result: i64 = 0;

    for node in connections.get(current_node).unwrap() {
        if node == "start" {
            continue;
        }
        let has_already_visited_small_cave_twice = visited.values().any(|v| *v > 1);
        if node.chars().all(|c| c.is_ascii_lowercase()) {
            if visited.contains_key(node) && has_already_visited_small_cave_twice {
                continue;
            }
        }
        result += calculate_entry_count(node, connections, visited.clone());
    }

    result
}
