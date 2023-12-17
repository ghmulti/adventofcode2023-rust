use std::collections::HashMap;
use priority_queue::PriorityQueue;

use pathfinding::{
    directed::dijkstra::dijkstra,
    matrix::{directions, Matrix},
};

const UP: u8 = 0;
const RIGHT: u8 = 1;
const DOWN: u8 = 2;
const LEFT: u8 = 3;

pub(crate) fn day17() {
    println!("Day 17");
    let file_content = include_str!("../resources/test-input.txt");
    // let file_content = include_str!("../resources/day17.txt").trim();
    // println!("File content:\n{}", file_content);

    let map: Vec<_> = file_content.lines().map(|line| {
        line.chars().map(|e| e.to_digit(10).unwrap() as usize).collect::<Vec<_>>()
    }).collect();
    // println!("Map: {:?}", map);

    __dijkstra(&map);
    _dijkstra(&map);
}

fn __dijkstra(map: &Vec<Vec<usize>>) {
    let start: (usize, usize) = (0, 0);
    let end: (usize, usize) = (map.len()-1, map[0].len()-1);
    let start_element = Element { position: start, direction: RIGHT, counter: 0 };
    let path = dijkstra(
        &start_element,
        |element| {
            let moves: Vec<(_, usize)> = possible_moves(&element, &map).iter().map(|e| {
                let heat = map[e.position.0][e.position.1];
                (e.clone(), heat)
            }).collect();
            moves
        },
        |element| element.position == end
    );
    println!("Least heat loss for end element: {:?}", path.unwrap().1)
}

fn _dijkstra(map: &Vec<Vec<usize>>) {
    let start: (usize, usize) = (0, 0);
    let end: (usize, usize) = (map.len()-1, map[0].len()-1);

    let start_element = Element { position: start, direction: RIGHT, counter: 0 };

    let mut distances: HashMap<Element, usize> = HashMap::new();
    distances.insert(start_element.clone(), 0);

    let mut elements_to_discover: PriorityQueue<Element, usize> = PriorityQueue::new();
    elements_to_discover.push((start_element), priority(0));

    while elements_to_discover.peek().unwrap().0.position != end {
        let (element, _) = elements_to_discover.pop().unwrap();
        let current_element_heat = *distances.get(&element).unwrap();
        let moves: Vec<_> = possible_moves(&element, &map);
        for m in moves {
            let move_heat = map[m.position.0][m.position.1];
            if !distances.contains_key(&m) || distances[&m] > current_element_heat + move_heat {
                distances.insert(m.clone(), current_element_heat + move_heat);
            }
            elements_to_discover.push(m, priority(current_element_heat + move_heat));
        }
    }


}

fn priority(heat: usize) -> usize {
    usize::MAX - heat
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Element { position: (usize, usize), direction: u8, counter: u8 }

fn possible_moves(el: &Element, map: &Vec<Vec<usize>>) -> Vec<Element> {
    let (row, column) = el.position;
    let mut result: Vec<Element> = vec![];
    // left
    if column > 0 && el.direction != RIGHT {
        let counter = if el.direction == LEFT { el.counter+1 } else { 1 };
        if counter <= 3 {
            result.push(Element { position: (row, column - 1), direction: LEFT, counter });
        }
    }
    // up
    if row > 0 && el.direction != DOWN  {
        let counter = if el.direction == UP { el.counter+1 } else { 1 };
        if counter <= 3 {
            result.push(Element { position: (row - 1, column), direction: UP, counter });
        }
    }
    // down
    if row < map.len()-1 && el.direction != UP {
        let counter = if el.direction == DOWN { el.counter+1 } else { 1 };
        if counter <= 3 {
            result.push(Element { position: (row + 1, column), direction: DOWN, counter });
        }
    }
    // right
    if column < map[0].len()-1 && el.direction != LEFT {
        let counter = if el.direction == RIGHT { el.counter+1 } else { 1 };
        if counter <= 3 {
            result.push(Element { position: (row, column + 1), direction: RIGHT, counter });
        }
    }
    result
}