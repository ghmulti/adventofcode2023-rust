use pathfinding::directed::dijkstra::dijkstra;

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

    __dijkstra(&map, RIGHT, possible_moves_1);
    __dijkstra(&map, DOWN, possible_moves_2);
}

fn __dijkstra(map: &Vec<Vec<usize>>, initial_direction: u8, possible_fn: fn(el: &Element, map: &Vec<Vec<usize>>) -> Vec<Element>) {
    let start: (usize, usize) = (0, 0);
    let end: (usize, usize) = (map.len()-1, map[0].len()-1);
    let start_element = Element { position: start, direction: initial_direction, counter: 0 };
    let path = dijkstra(
        &start_element,
        |element| {
            let moves: Vec<(_, usize)> = possible_fn(&element, &map).iter().map(|e| {
                let heat = map[e.position.0][e.position.1];
                (e.clone(), heat)
            }).collect();
            moves
        },
        |element| element.position == end
    );
    // print!("{:?}\n", path.clone().unwrap().0.iter().map(|e| e.position).collect::<Vec<_>>());
    println!("Least heat loss for end element: {:?}", path.unwrap().1)
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Element { position: (usize, usize), direction: u8, counter: u8 }

fn possible_moves_1(el: &Element, map: &Vec<Vec<usize>>) -> Vec<Element> {
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
    // left
    if column > 0 && el.direction != RIGHT {
        let counter = if el.direction == LEFT { el.counter+1 } else { 1 };
        if counter <= 3 {
            result.push(Element { position: (row, column - 1), direction: LEFT, counter });
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

fn possible_moves_2(el: &Element, map: &Vec<Vec<usize>>) -> Vec<Element> {
    let (row, column) = el.position;
    if el.counter >= 4 {
        let mut result: Vec<Element> = vec![];
        // up
        if row > 0 && el.direction != DOWN {
            let counter = if el.direction == UP { el.counter + 1 } else { 1 };
            if counter <= 10 {
                result.push(Element { position: (row - 1, column), direction: UP, counter });
            }
        }
        // down
        if row < map.len() - 1 && el.direction != UP {
            let counter = if el.direction == DOWN { el.counter + 1 } else { 1 };
            if counter <= 10 {
                result.push(Element { position: (row + 1, column), direction: DOWN, counter });
            }
        }
        // left
        if column > 0 && el.direction != RIGHT {
            let counter = if el.direction == LEFT { el.counter + 1 } else { 1 };
            if counter <= 10 {
                result.push(Element { position: (row, column - 1), direction: LEFT, counter });
            }
        }
        // right
        if column < map[0].len() - 1 && el.direction != LEFT {
            let counter = if el.direction == RIGHT { el.counter + 1 } else { 1 };
            if counter <= 10 {
                result.push(Element { position: (row, column + 1), direction: RIGHT, counter });
            }
        }
        result
    } else {
        match el.direction {
            UP => {
                if el.position.0 > 0 {
                    vec![Element { position: (el.position.0 - 1, el.position.1), direction: el.direction, counter: el.counter + 1 }]
                } else {
                    vec![]
                }
            }
            DOWN => {
                if el.position.0 < map.len() - 1 {
                    vec![Element { position: (el.position.0 + 1, el.position.1), direction: el.direction, counter: el.counter + 1 }]
                } else {
                    vec![]
                }
            }
            LEFT => {
                if el.position.1 > 0 {
                    vec![Element { position: (el.position.0, el.position.1 - 1), direction: el.direction, counter: el.counter + 1 }]
                } else {
                    vec![]
                }
            }
            RIGHT => {
                if el.position.1 < map[0].len() - 1 {
                    vec![Element { position: (el.position.0, el.position.1 + 1), direction: el.direction, counter: el.counter + 1 }]
                } else {
                    vec![]
                }
            }
            _ => panic!("🤯")
        }
    }
}
