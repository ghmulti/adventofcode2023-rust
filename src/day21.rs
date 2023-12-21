use std::collections::HashSet;
use std::hash::Hash;
use std::time::Instant;

pub(crate) fn day21() {
    println!("Day 21");
    let file_content = include_str!("../resources/test-input.txt");
    // let file_content = include_str!("../resources/day21.txt").trim();
    // println!("File content:\n{}", file_content);

    // let map = format_map(file_content.lines().collect::<Vec<_>>());
    let map: Vec<_> = file_content.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    // println!("Map: {:?}", map);

    let start_vec = map.iter().enumerate().filter_map(|(row_index, row)| {
        row.iter().enumerate().find(|(_, &ch)| ch == 'S').map(|(column_index, _)| (row_index as isize, column_index as isize))
    }).collect::<Vec<_>>();
    let start = start_vec.first().unwrap();
    println!("Start point: {:?}", start);

    part_1(start, &map);
    let now = Instant::now();
    part_2(start, &map);
    println!("Part 2 took {}secs", now.elapsed().as_secs());
}

fn part_1(start: &(isize, isize), map: &Vec<Vec<char>>) {
    let mut positions: HashSet<(isize, isize, isize, isize)> = HashSet::new();
    positions.insert((start.0, start.1, 0, 0));
    // println!("{:?}", find_movement(*start, &map));
    for _ in 0..64 {
        let new_positions = positions.iter().fold(HashSet::new(), |mut acc, p| {
            let res = find_movement((p.0, p.1, 0, 0), &map);
            acc.extend(res);
            acc
        });
        positions = new_positions;
    }
    // println!("Positions: {:?}", positions);
    println!("Number of positions for part 1={}", positions.len());
    // visualize(&positions, &map);
}

fn part_2(start: &(isize, isize), map: &Vec<Vec<char>>) {
    let mut positions: HashSet<(isize, isize, isize, isize)> = HashSet::new();
    positions.insert((start.0, start.1, 0, 0));
    // println!("{:?}", find_movement(*start, &map));
    for index in 0..100 {
        let new_positions = positions.iter().fold(HashSet::new(), |mut acc, p| {
            let res = find_movement(*p, &map);
            acc.extend(res);
            acc
        });

        // let p1 = positions.clone().iter().map(|e| (e.0, e.1)).collect::<HashSet<_>>();
        // let p11 = positions.clone().iter().map(|e| (e.2, e.3)).collect::<HashSet<_>>();
        // let p2 = new_positions.clone().iter().map(|e| (e.0, e.1)).collect::<HashSet<_>>();
        // let p22 = new_positions.clone().iter().map(|e| (e.2, e.3)).collect::<HashSet<_>>();
        // if p1 == p2 {
        //     println!("Got match for index {}, p1 len {} with unique tiles {} p2 len {} with unique tiles {}", index, p1.len(), p11.len(), p2.len(), p22.len());
        // }

        positions = new_positions;
    }

    // repeat after 20 element (1 + 4 + 6)
    let loops = 26501365 - 20;
    let number_of_loops = loops / 11;
    println!("{}", number_of_loops);

    let mut result = 216; // 0-20 loop
    for i in 0..number_of_loops {

    }

    // println!("Positions: {:?}", positions);
    println!("Number of positions for part 2={}", positions.len());
    // visualize(&positions, &map);
}

const N: (isize, isize) = (-1, 0);
const S: (isize, isize) = (1, 0);
const W: (isize, isize) = (0, -1);
const E: (isize, isize) = (0, 1);

fn find_movement(pos: (isize, isize, isize, isize), map: &Vec<Vec<char>>) -> Vec<(isize, isize, isize, isize)> {
    let positions = [N, S, W, E].iter()
        .map(|(x, y)| {
            (pos.0 + x, pos.1 + y, pos.2, pos.3)
        })
        .map(|(x, y, tile_x, tile_y)| {
            // println!("Current coords {} {} {} {}", x, y, tile_x,tile_y);
            let new_x: isize = if x < 0 { map.len() as isize + x } else { x % map.len() as isize };
            let new_y: isize = if y < 0 { map[0].len() as isize + y } else { y % map[0].len() as isize };
            let mut new_tile_x = tile_x;
            if x < 0 {
                new_tile_x -= 1;
            } else if x > (map.len()-1) as isize {
                new_tile_x += 1;
            }
            let mut new_tile_y = tile_y;
            if y < 0 {
                new_tile_y -= 1;
            } else if y > (map[0].len()-1) as isize {
                new_tile_y += 1;
            }
            // println!("New coords {} {} {} {}", new_x, new_y, new_tile_x, new_tile_y);
            (new_x, new_y, new_tile_x, new_tile_y)
        })
        .filter(|&(x, y, tile_x, tile_y)| {
            map[x as usize][y as usize] != '#'
        })
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