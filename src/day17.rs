use std::collections::HashMap;

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

    let start: (usize, usize) = (0, 0);
    let mut heat_cache: HashMap<(usize,usize), usize> = HashMap::new();
    let start_element = Element { position: start, direction: RIGHT, counter: 0 };

    let mut elements_to_discover: Vec<(Element, Vec<(usize, usize)>)> = vec![(start_element, vec![(0, 0)])];
    while elements_to_discover.len() > 0 {
        let (element, visited) = elements_to_discover.pop().unwrap();
        let current_heat = visited.iter().filter(|p| *p != &(0usize, 0usize)).map(|p| map[p.0][p.1]).sum::<usize>();
        let moves: Vec<_> = possible_moves(&element, &map);
        // println!("Checking element: {:?}, possible moves: {:?}", element, moves);
        for m in moves {
            let heat = map[m.position.0][m.position.1];
            let accumulated_heat = current_heat + heat;
            if heat_cache.contains_key(&m.position) && *heat_cache.get(&m.position).unwrap() < accumulated_heat {
                continue
            }
            if !visited.contains(&m.position) {
                if !heat_cache.contains_key(&m.position) || *heat_cache.get(&m.position).unwrap() >= accumulated_heat {
                    heat_cache.insert(m.position, accumulated_heat);
                }
                let mut cloned_visited = visited.clone();
                cloned_visited.push(m.position);
                elements_to_discover.push((m, cloned_visited));
            }
        }
    }

    // recursive_dfs(&start_element, 0, &vec![(0, 0)], &map, &mut heat_cache);

    let end = (map.len()-1, map[0].len()-1);
    println!("Minimal heat loss: {}", heat_cache.get(&end).unwrap());
}

fn recursive_dfs(e: &Element, accumulated_heat: usize, visited: &Vec<(usize, usize)>, map: &Vec<Vec<usize>>, heat_cache: &mut HashMap<(usize, usize), usize>) {
    let end = (map.len()-1, map[0].len()-1);
    let current_heat = if e.position == (0, 0) { 0 } else { map[e.position.0][e.position.1] };
    let total_heat = accumulated_heat+current_heat;
    if !heat_cache.contains_key(&e.position) || *heat_cache.get(&e.position).unwrap() > total_heat {
        heat_cache.insert(e.position, total_heat);
    }
    if e.position == end {
        println!("Reached target position! Current minimum: {}, Cache size: {}", heat_cache.get(&e.position).unwrap(), heat_cache.len());
        return;
    }
    let moves = possible_moves(&e, &map);
    let filtered_moves: Vec<_> = moves.iter().filter(|e| !visited.contains(&e.position)).collect();
    for m in filtered_moves {
        if heat_cache.contains_key(&m.position) && *heat_cache.get(&m.position).unwrap() < total_heat  {
            continue
        }
        let mut cloned_visited = visited.clone();
        cloned_visited.push(m.position);
        recursive_dfs(m, accumulated_heat+current_heat, &cloned_visited, map, heat_cache);
    }
}

#[derive(Debug)]
struct Element { position: (usize, usize), direction: u8, counter: u8 }

fn possible_moves(el: &Element, map: &Vec<Vec<usize>>) -> Vec<Element> {
    let (row, column) = el.position;
    let mut result: Vec<Element> = vec![];
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
    // left
    if column > 0 && el.direction != RIGHT {
        let counter = if el.direction == LEFT { el.counter+1 } else { 1 };
        if counter <= 3 {
            result.push(Element { position: (row, column - 1), direction: LEFT, counter });
        }
    }
    result
}