use std::collections::HashSet;

pub(crate) fn day21() {
    println!("Day 21");
    // let file_content = include_str!("../resources/test-input.txt");
    let file_content = include_str!("../resources/day21.txt").trim();
    // println!("File content:\n{}", file_content);

    // let map = format_map(file_content.lines().collect::<Vec<_>>());
    let map: Vec<_> = file_content.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    // println!("Map: {:?}", map);

    let start_vec = map.iter().enumerate().filter_map(|(index, row)| {
        row.iter().enumerate().find(|(_, &ch)| ch == 'S').map(|(column, _)| (index, column))
    }).collect::<Vec<_>>();
    let start = start_vec.first().unwrap();
    println!("Start point: {:?}", start);

    part_1(start, &map);
}

fn part_1(start: &(usize, usize), map: &Vec<Vec<char>>) {
    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    positions.insert(*start);
    println!("{:?}", find_movement(*start, &map));
    for _ in 0..64 {
        let new_positions = positions.iter().fold(HashSet::new(), |mut acc, p| {
            let res = find_movement(*p, &map);
            acc.extend(res);
            acc
        });
        positions = new_positions;
    }
    // println!("Positions: {:?}", positions);
    println!("Number of positions={}", positions.len());
    // visualize(&positions, &map);
}

const N: (isize, isize) = (-1, 0);
const S: (isize, isize) = (1, 0);
const W: (isize, isize) = (0, -1);
const E: (isize, isize) = (0, 1);

fn find_movement(pos: (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let positions = [N, S, W, E].iter().map(|(x, y)| {
        (pos.0 as isize - x, pos.1 as isize - y)
    }).filter(|(x, y)| {
        *x > 0 && *x < (map.len()-1) as isize && *y > 0 && *y < (map[0].len()-1) as isize
    }).map(|(x, y)| (x as usize, y as usize))
        .filter(|&(x, y)| map[x][y] != '#')
        .collect::<Vec<_>>();

    positions
}

fn visualize(elements: &HashSet<(usize, usize)>, map: &Vec<Vec<char>>) {
    for (row, line) in map.iter().enumerate() {
        let mut newline = String::new();
        for (column, _) in line.iter().enumerate() {
            if elements.contains(&(row, column)) {
                newline.push('O');
            } else {
                newline.push(map[row][column]);
            }
        }
        println!("{}", newline);
    }
}

fn format_map(lines: Vec<&str>) -> Vec<Vec<char>> {
    let mut map: Vec<_> = lines.iter().map(|line| line.chars().collect::<Vec<_>>()).collect();

    // adding one line of dots in beginning and end
    let dots: Vec<char> = (0..map[0].len()).map(|_| '.').collect();
    map.insert(0, dots.clone());
    map.push(dots.clone());

    // adding column of dots in beginning and end of line
    for e in map.iter_mut() {
        e.insert(0, '.');
        e.push('.');
    };

    map
}