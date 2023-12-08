use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::iter::Cycle;

use num::integer::lcm;

pub(crate) fn day8() {
    println!("Day 8");
    let file_path = "resources/day08.txt";
    // let file_path = "resources/test-input.txt";
    let mut file = File::open(file_path).expect("Unable to open file");

    let mut buffer = String::new();
    file.read_to_string( &mut buffer).expect("Unable to read file into string");
    // println!("File contents:\n{}", buffer);

    let lines: Vec<String> = buffer.lines().map(String::from).collect();

    let directions :Vec<_> = lines[0].chars().collect();
    // println!("Directions: {:?}", directions);

    let nodes: Vec<_> = lines[2..].iter().map(|line| { parse_node(line) }).collect();
    let node_map: HashMap<&str, (&str, &str)> = nodes.clone().into_iter().collect();
    // println!("Node map: {:?}", node_map);

    part_1(&node_map, &directions);
    part_2(nodes, &node_map, &directions);
}

fn part_2(nodes: Vec<(&str, (&str, &str))>, node_map: &HashMap<&str, (&str, &str)>, directions: &Vec<char>) {
    fn pred(e: &str) -> bool { !e.ends_with('Z') }
    let states: Vec<(&str, i64)> = nodes.iter()
        .filter(|(e, _)| e.ends_with('A'))
        .map(|(e, _)| { (*e, 0) }).collect();
    // println!("States: {:?}", states);
    let steps_for_states: Vec<_> = states.iter().map(|state| {
        find_target_step(*state, pred, &node_map, directions)
    }).collect();
    println!("Steps for states to reach '--Z': {:?}", steps_for_states);
    let lcm = steps_for_states.iter().fold(1, |acc, e| { lcm(acc, *e) });
    println!("Least Common Multiple for steps: {:?}", lcm);
}

fn part_1(node_map: &HashMap<&str, (&str, &str)>, directions: &Vec<char>) {
    let state:(&str, i64)  = ("AAA", 0);
    fn pred(e: &str) -> bool { e != "ZZZ" }
    let number_of_steps = find_target_step(state, pred, &node_map, directions);
    println!("Steps to reach ZZZ: {}", number_of_steps);
}

fn find_target_step<'a>(mut state: (&'a str, i64), pred : fn(&str) -> bool, node_map: &HashMap<&'a str, (&'a str, &'a str)>, directions: &Vec<char>) -> i64 {
    let mut cycled_array_iterator : Cycle<_> = directions.iter().cycle();
    while pred(state.0) {
        let (l, r) = node_map[state.0];
        let next_node = cycled_array_iterator.next().unwrap();
        state.0 = match next_node {
            'L' => l,
            'R' => r,
            _ => panic!("not expected")
        };
        state.1 = state.1 + 1;
        // println!("Current state: {:?}", state);
    }
    state.1
}

fn parse_node(line: &str) -> (&str, (&str, &str)) {
    (&line[0..3], (&line[7..10], &line[12..15]))
}

#[cfg(test)]
mod tests {
    use crate::day08::parse_node;

    #[test]
    fn check() {
        assert_eq!(parse_node("AAA = (BBB, CCC)"), ("AAA", ("BBB", "CCC")));
    }
}