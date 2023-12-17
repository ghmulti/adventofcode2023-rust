use std::collections::HashMap;

const UP: u8 = 0;
const RIGHT: u8 = 1;
const DOWN: u8 = 2;
const LEFT: u8 = 3;

pub(crate) fn day17() {
    println!("Day 17");
    // let file_content = include_str!("../resources/test-input.txt");
    let file_content = include_str!("../resources/day17.txt").trim();
    // println!("File content:\n{}", file_content);

    let map: Vec<_> = file_content.lines().map(|line| {
        line.chars().map(|e| e.to_digit(10).unwrap() as usize).collect::<Vec<_>>()
    }).collect();
    // println!("Map: {:?}", map);

    let start: (usize, usize) = (0, 0);
    let mut heat_cache: HashMap<(usize, usize, u8, u8), usize> = HashMap::new();
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
            let cache_key = (m.position.0, m.position.1, m.direction, m.counter);
            if heat_cache.contains_key(&cache_key) && *heat_cache.get(&cache_key).unwrap() < accumulated_heat {
                continue
            }
            if m.position == (map.len()-1, map[0].len()-1) {
                println!("Reached target, current min={}, cache min {:?}", accumulated_heat, find_end_min(&map, &heat_cache).iter().min());
            }
            if !visited.contains(&m.position) {
                if !heat_cache.contains_key(&cache_key) || *heat_cache.get(&cache_key).unwrap() > accumulated_heat {
                    heat_cache.insert(cache_key, accumulated_heat);
                }
                let mut cloned_visited = visited.clone();
                cloned_visited.push(m.position);
                elements_to_discover.push((m, cloned_visited));
            }
        }
    }

    // recursive_dfs(&start_element, 0, &vec![(0, 0)], &map, &mut heat_cache);
    println!("Minimal heat loss: {:?}", find_end_min(&map, &heat_cache).iter().min());
}

fn find_end_min(map: &Vec<Vec<usize>>, heat_cache: &HashMap<(usize, usize, u8, u8), usize>) -> Vec<usize> {
    let results: Vec<_> = [RIGHT, LEFT, UP, DOWN].iter().flat_map(|direction| {
        let r: Vec<_> = (1..=3).map(|counter| (*direction, counter)).collect();
        r
    }).map(|(direction, counter)| {
        let cache_key = (map.len() - 1, map[0].len() - 1, direction, counter);
        heat_cache.get(&cache_key)
    }).filter(|e| e.is_some()).map(|e| *e.unwrap()).collect();
    results
}

#[derive(Debug)]
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

// min=3991, high
// min=1474, high
// min=1406, high