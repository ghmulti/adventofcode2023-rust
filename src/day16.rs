use std::collections::HashSet;

const UP: u8 = 0;
const RIGHT: u8 = 1;
const DOWN: u8 = 2;
const LEFT: u8 = 3;

pub(crate) fn day16() {
    println!("Day 16");
    // let file_content = include_str!("../resources/test-input.txt");
    let file_content = include_str!("../resources/day16.txt").trim();
    // println!("File content:\n{}", file_content);

    let map: Vec<_> = file_content.lines().map(|line| line.chars().collect::<Vec<_>>()).collect();
    // println!("Map: {:?}", map);

    let beams = part_1((0,0), RIGHT, &map);
    let elements: Vec<_> = covered_elements(&beams);
    // println!("{}", visualize(&covered_elements, &map));
    println!("Tiles energized from (0,0) RIGHT: {}", elements.len());

    let edges1: Vec<(usize, usize, u8)> = (0..map.len()).map(|row| (row, 0, RIGHT)).collect();
    let edges2: Vec<(usize, usize, u8)> = (0..map.len()).map(|row| (row, map[0].len()-1, LEFT)).collect();
    let edges3: Vec<(usize, usize, u8)> = (0..map[0].len()).map(|column| (0, column, DOWN)).collect();
    let edges4: Vec<(usize, usize, u8)> = (0..map[0].len()).map(|column| (map.len()-1, column, UP)).collect();
    let max_length = ([edges1, edges2, edges3, edges4].concat()).iter().fold(0, |max, (row, column, direction)| {
        let beams = part_1((*row, *column), *direction, &map);
        let covered_length = covered_elements(&beams).len();
        if covered_length > max {
            covered_length
        } else {
            max
        }
    });
    println!("Max number of energized tiles {}", max_length);
}

fn part_1(init_position: (usize, usize), init_direction: u8, map: &Vec<Vec<char>>) -> Vec<Beam> {
    let (direction, _) = new_directions(init_direction, map[init_position.0][init_position.1]);
    let mut beams: Vec<Beam> = vec![
        Beam { path: vec![(init_position.0, init_position.1, direction)], active: true },
    ];
    while beams.iter().any(|beam| beam.active) {
        for index in 0..beams.len() {
            if !beams[index].active {
                continue
            }
            let mut beam: Beam = beams[index].clone();
            let current_path_element = beam.path.last().unwrap();
            let new_coords = move_beam(*current_path_element, &map);
            if new_coords.is_none() {
                beam.active = false;
                beams[index] = beam;
                continue
            }

            let (new_row, new_column) = new_coords.unwrap();
            let (new_direction_1, new_direction_2) = new_directions(current_path_element.2, map[new_row][new_column]);
            let new_path_element = (new_row, new_column, new_direction_1);
            if beams.iter().any(|beam| beam.path.contains(&new_path_element)) {
                beam.active = false;
            } else {
                beam.path.push(new_path_element);
            }
            beams[index] = beam;

            if new_direction_2.is_some() {
                let new_path_element = (new_row, new_column, new_direction_2.unwrap());
                if !beams.iter().any(|beam| beam.path.contains(&new_path_element)) {
                    beams.push(Beam { path: vec![new_path_element], active: true });
                }
            }
        }
    }
    beams
}

fn covered_elements(beams: &Vec<Beam>) -> Vec<(usize, usize)> {
    let covered_elements: Vec<_> = beams.iter().fold(HashSet::new(), |mut acc, el| {
        for e in &el.path {
            acc.insert((e.0, e.1));
        }
        acc
    }).into_iter().collect();
    covered_elements
}

fn visualize(covered: &Vec<(usize, usize)>, map: &Vec<Vec<char>>) -> String {
    let mut lines: Vec<String> = vec![];
    for (row, line) in map.iter().enumerate() {
        let mut newline = String::new();
        for (column, _) in line.iter().enumerate() {
            if covered.contains(&(row, column)) {
                newline.push('#');
            } else {
                newline.push('.');
            }
        }
        lines.push(newline);
    }
    lines.join("\n")
}

fn new_directions(current_direction: u8, ch: char) -> (u8, Option<u8>) {
    if ch == '.'{
        (current_direction, None)
    } else if ch == '/' {
        let res = match current_direction {
            UP => { RIGHT }
            RIGHT => { UP }
            DOWN => { LEFT }
            LEFT => { DOWN }
            _ => { panic!("🤯") }
        };
        (res, None)
    } else if ch == '\\' {
        let res = match current_direction {
            UP => { LEFT }
            RIGHT => { DOWN }
            DOWN => { RIGHT }
            LEFT => { UP }
            _ => { panic!("🤯") }
        };
        (res, None)
    } else if ch == '-' {
        match current_direction {
            LEFT => { (current_direction, None) }
            RIGHT => { (current_direction, None) }
            UP => { (LEFT, Some(RIGHT)) }
            DOWN => { (LEFT, Some(RIGHT)) }
            _ => { panic!("🤯") }
        }
    } else if ch == '|' {
        match current_direction {
            LEFT => { (UP, Some(DOWN)) }
            RIGHT => { (UP, Some(DOWN)) }
            UP => { (current_direction, None) }
            DOWN => { (current_direction, None) }
            _ => { panic!("🤯") }
        }
    } else {
        panic!("unsure")
    }
}

fn move_beam((row, column, direction): (usize, usize, u8), map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let new_position: Option<(usize, usize)> = match direction {
        UP => {
            if row > 0 {
                Some((row-1, column))
            } else {
                None
            }
        }
        RIGHT => {
            if column < map[0].len()-1 {
                Some((row, column+1))
            } else {
                None
            }
        }
        DOWN => {
            if row < map.len()-1 {
                Some((row+1, column))
            } else {
                None
            }
        }
        LEFT => {
            if column > 0 {
                Some((row, column-1))
            } else {
                None
            }
        }
        _ => { panic!("🤯") }
    };
    new_position
}

#[derive(Clone,Debug)]
struct Beam {
    path: Vec<(usize, usize, u8)>,
    active: bool,
}