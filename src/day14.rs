use std::collections::HashMap;
use std::time::Instant;

pub(crate) fn day14() {
    println!("Day 14");
    // let file_content = include_str!("../resources/test-input.txt");
    let file_content = include_str!("../resources/day14.txt");
    // println!("File content:\n{}", file_content);
    let lines: Vec<_> = file_content.lines().collect();
    let map: Vec<Vec<char>> = lines.iter().map(|e| e.chars().collect()).collect();
    // println!("Map: {:?}", map);

    let map_with_moved_rocks = part_1(&map);
    let score = calculate_score(&map_with_moved_rocks);
    println!("Total load on the north support beam: {}", score);

    let now = Instant::now();
    let map_with_moved_rocks = part_2(&map);
    let score = calculate_score(&map_with_moved_rocks);
    println!("Total load on the north support beam after 1000000000 cycles: {}, {}secs", score, now.elapsed().as_secs());
}

fn part_2(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut cloned_map: Vec<_> = rotate_cycle(&map.clone());
    let mut cache : HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let mut counter: usize = 1;
    while !cache.contains_key(&*cloned_map) {
        cache.insert(cloned_map.clone(), counter);
        cloned_map = rotate_cycle(&cloned_map);
        counter += 1;
    }
    // println!("Found cycle {}, {}", counter, cache[&*cloned_map]);
    let loop_start = cache[&*cloned_map];
    let step = counter-loop_start;
    let remaining = (1000000000-loop_start) % step;
    let mut result = map.clone();
    for _ in 0..(loop_start+remaining) {
        result = rotate_cycle(&result);
    }
    result
}

fn rotate_cycle(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut cloned_map: Vec<_> = map.clone();
    for _ in 0..4 {
        cloned_map = part_1(&cloned_map);
        cloned_map = rotate(&cloned_map);
    }
    // display_map(&cloned_map);
    cloned_map
}

fn part_1(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rocks: Vec<_> = find_rocks(&map);
    // println!("Rocks: {:?}", rocks);
    let mut cloned_map: Vec<_> = map.clone();
    rocks.iter().fold(&mut cloned_map, |map, rock| {
        move_rock(rock, map);
        map
    });
    cloned_map
}

#[allow(dead_code)]
fn display_map(map: &Vec<Vec<char>>) {
    println!("\nMap preview:\n{}", map.iter().map(|e| e.into_iter().collect::<String>()).collect::<Vec<String>>().join("\n"))
}

fn rotate(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..map[0].len()).map(|column| {
        let chars: Vec<_> = (0..map.len()).rev().map(|row| {
            map[row][column]
        }).collect();
        chars
    }).collect()
}

fn calculate_score(map: &Vec<Vec<char>>) -> usize {
    let mut score: usize = 0;
    for (row, line) in map.iter().enumerate() {
        let ch: Vec<_> = line.iter().filter(|e| **e == 'O').collect();
        score += ch.len() * (map.len() - row);
    }
    score
}

fn move_rock(rock_position: &(usize, usize), map: &mut Vec<Vec<char>>) {
    // println!("Moving {:?}", rock_position);
    let mut cloned_position = rock_position.clone();
    while cloned_position.0 > 0 && map[cloned_position.0-1][cloned_position.1] == '.' {
        cloned_position.0 -= 1;
    }
    map[rock_position.0][rock_position.1] = '.';
    map[cloned_position.0][cloned_position.1] = 'O';
}

fn find_rocks(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = vec![];
    for (row, line) in map.iter().enumerate() {
        for (column, char) in line.iter().enumerate() {
            if *char == 'O' {
                res.push((row, column))
            }
        }
    }
    res
}