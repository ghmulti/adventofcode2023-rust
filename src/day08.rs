use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::iter::Cycle;

pub(crate) fn day8() {
    println!("Day 8");
    let file_path = "resources/day08.txt";
    // let file_path = "resources/test-input.txt";
    let mut file = File::open(file_path).expect("Unable to open file");

    let mut buffer = String::new();
    file.read_to_string( &mut buffer).expect("Unable to read file into string");
    println!("File contents:\n{}", buffer);

    let lines: Vec<String> = buffer.lines().map(String::from).collect();

    let directions :Vec<_> = lines[0].chars().collect();
    println!("Directions: {:?}", directions);

    let nodes: Vec<_> = lines[2..].iter().map(|line| { parse_node(line) }).collect();
    let node_map: HashMap<&str, (&str, &str)> = nodes.clone().into_iter().collect();
    println!("Node map: {:?}", node_map);

    // part_1(node_map, &directions);
    part_2(nodes, node_map, &directions);
}

fn part_2(nodes: Vec<(&str, (&str, &str))>, node_map: HashMap<&str, (&str, &str)>, directions: &Vec<char>) {
    let mut cycled_array_iterator : Cycle<_> = directions.iter().cycle();
    let mut states: Vec<(&str, i64)> = nodes.iter()
        .filter(|(e, _)| e.ends_with('A'))
        .map(|(e, _)| { (*e, 0) }).collect();
    println!("States: {:?}", states);
    while states.iter().any(|(e, _)| !e.ends_with('Z')) {
        let next_node = cycled_array_iterator.next().unwrap();
        for i in 0..states.len() {
            let state = states.get_mut(i).unwrap();
            let (l, r) = node_map[state.0];
            state.0 = match next_node {
                'L' => l,
                'R' => r,
                _ => panic!("not expected")
            };
            state.1 = state.1 + 1;
            // println!("Changing state: {:?}", state);
        }
        println!("Current states: {:?}", states);
    }
}

fn part_1(node_map: HashMap<&str, (&str, &str)>, directions: &Vec<char>) {
    let mut cycled_array_iterator : Cycle<_> = directions.iter().cycle();
    let mut state:(&str, i64)  = ("AAA", 0);
    while state.0 != "ZZZ" {
        let (l, r) = node_map[state.0];
        let next_node = cycled_array_iterator.next().unwrap();
        state.0 = match next_node {
            'L' => l,
            'R' => r,
            _ => panic!("not expected")
        };
        state.1 = state.1 + 1;
        println!("Current state: {:?}", state);
    }
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