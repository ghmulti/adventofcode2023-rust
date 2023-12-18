use std::collections::HashMap;
use priority_queue::PriorityQueue;

const N: (isize, isize) = (-1, 0);
const S: (isize, isize) = (1, 0);
const W: (isize, isize) = (0, -1);
const E: (isize, isize) = (0, 1);

pub(crate) fn day17_v2() {
    println!("Day 17 v2");
    let file_content = include_str!("../resources/test-input.txt");
    // let file_content = include_str!("../resources/day17.txt").trim();
    println!("File content:\n{}", file_content);

    let map: Vec<_> = file_content.lines().map(|line| {
        line.chars().map(|e| e.to_digit(10).unwrap() as usize).collect::<Vec<_>>()
    }).collect();
    // println!("Map: {:?}", map);

    let start: (isize, isize) = (0, 0);
    let end: (isize, isize) = ((map.len() - 1) as isize, (map[0].len() - 1) as isize);
    let start_element: Node = Node { position: start.clone(), direction: E, counter: 0 };

    let mut elements_to_discover: PriorityQueue<Node, usize> = PriorityQueue::new();
    elements_to_discover.push(start_element, usize::MAX);

    let mut distances: HashMap<(isize, isize), usize> = HashMap::new();
    distances.insert(start.clone(), 0);

    let mut visited: Vec<Node> = vec![];
    while elements_to_discover.peek().unwrap().0.position != end {
        let (node, priority) = elements_to_discover.pop().unwrap();
        let heat = usize::MAX - priority;
        if visited.contains(&node) {
            continue
        }
        visited.push(node.clone());
        let connections: Vec<_> = possible_moves(&node, &map);
        for connection in connections {
            let move_heat: usize = map[connection.position.0 as usize][connection.position.1 as usize];
            let accumulated_heat: usize = heat + move_heat;
            if !distances.contains_key(&connection.position) || *distances.get(&connection.position).unwrap() > accumulated_heat {
                distances.insert(connection.position.clone(), accumulated_heat);
            }
            elements_to_discover.push(connection, usize::MAX - accumulated_heat);
            // if distance_to_end(&connection.position, &end) < distance_to_end(&node.position, &end) {
            //     elements_to_discover.push(connection, usize::MAX - accumulated_heat);
            // }
        }
    }

    println!("Min end distance: {:?}", distances.get(&end));
}

fn distance_to_end(position: &(isize, isize), end: &(isize, isize)) -> usize {
    (end.0 - position.0 + end.1 - position.1) as usize
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Node {
    position: (isize, isize),
    direction: (isize, isize),
    counter: usize,
}

fn possible_moves(node: &Node, map: &Vec<Vec<usize>>) -> Vec<Node> {
    [N, S, W, E].iter().map(|&direction| {
        Node {
            position: (node.position.0+direction.0, node.position.1+direction.1),
            direction,
            counter: if direction == node.direction { node.counter+1 } else { 1 }
        }
    })
        .filter(|new_node| {
            new_node.position.0 >= 0 && new_node.position.0 < map.len() as isize && new_node.position.1 >= 0 && new_node.position.1 < map[0].len() as isize
        })
        .filter(|new_node| {
            new_node.counter <= 4 && new_node.direction != (-node.direction.0, -node.direction.1)
        }).collect::<Vec<Node>>()
}