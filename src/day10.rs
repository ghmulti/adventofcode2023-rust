use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

pub(crate) fn day10() {
    println!("Day 10");
    // let file_path = "resources/day10.txt";
    let file_path = "resources/test-input.txt";
    let mut file = File::open(file_path).expect("Unable to open file");

    let mut buffer = String::new();
    file.read_to_string( &mut buffer).expect("Unable to read file into string");
    // println!("File contents:\n{}", buffer);

    let lines: Vec<String> = buffer.lines().map(String::from).collect();
    let map: Vec<_> = lines.iter().map(|line| line.chars().collect::<Vec<_>>()).collect();
    // println!("Chars: {:?}", map);

    let start_position = find_start('S', &map);
    let mut path: Vec<_> = part_1(start_position, &map);
    println!("Path length: {:?}", path.len());
    println!("Steps to farthest position: {:?}", (path.len() as f32/2.0).ceil() as usize);

    path.push(start_position);
    visualize("resources/day10-vis.txt", &path, &map).unwrap();
    let not_enclosed : Vec<_> = part_2((0, 0), &map, &path);
    visualize("resources/day10-vis-enc.txt", &not_enclosed, &map).unwrap();
    let not_enclosed_2: Vec<_> = part_2((38, 0), &map, &path);
    visualize("resources/day10-vis-enc2.txt", &not_enclosed_2, &map).unwrap();

    let mut nested: Vec<(usize, usize)> = vec![];
    for (row, line) in map.iter().enumerate() {
        for (column, char) in line.iter().enumerate() {
            let pos = &(row, column);
            if !path.contains(pos) && !not_enclosed.contains(pos) && !not_enclosed_2.contains(pos) {
                nested.push((row, column))
            }
        }
    }
    visualize("resources/day10-vis-nested.txt", &nested, &map).unwrap();
    println!("Number of enclosed: {}", nested.len());
}

fn visualize(filename: &str, path: &Vec<(usize, usize)>, map: &Vec<Vec<char>>) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for (row, line) in map.iter().enumerate() {
        let mut newline = String::new();
        for (column, char) in line.iter().enumerate() {
            if path.contains(&(row, column)) {
                newline.push('x');
            } else {
                newline.push('_');
            }
        }
        file.write_all(newline.as_bytes())?;
        file.write_all(b"\n")?;
    }
    file.flush()?;

    Ok(())
}

fn part_1(start_position: (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    println!("Start position: {:?}", start_position);
    let connected_pipes: Vec<_> = find_connected_pipes(start_position, &map);
    // println!("Connected pipes: {:?}, {:?}", connected_pipes, find_chars(&connected_pipes, &map));


    let mut current_position: (usize, usize) = connected_pipes[0];
    let target_position: (usize, usize) = connected_pipes[1];
    let mut path: Vec<(usize, usize)> = vec![current_position];
    let mut visited: Vec<(usize, usize)> = vec![start_position];
    while current_position != target_position {
        // println!("Checking new position: {:?}, {:?}", current_position, map[current_position.0][current_position.1]);
        let connected_pipes: Vec<_> = find_connected_pipes(*path.last().unwrap(), &map);
        let not_visited_connected_pipes: Vec<_> = connected_pipes.iter().cloned().filter(|e| !visited.contains(e)).collect();
        // println!("Found not visited positions: {:?}, {:?}", not_visited_connected_pipes, find_chars(&not_visited_connected_pipes, &map));
        if not_visited_connected_pipes.is_empty(){
            path.pop();
        } else {
            current_position = not_visited_connected_pipes[0];
            path.push(current_position);
            visited.push(current_position);
        }
    }

    path
}

fn part_2(start_position: (usize, usize), map: &Vec<Vec<char>>, existing_path: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    println!("Start position: {:?}", start_position);
    let mut path: Vec<(usize, usize)> = vec![start_position];
    let mut visited: HashSet<(usize, usize)> = vec![].into_iter().collect();
    while !path.is_empty() {
        let last_position = path.last().unwrap();
        let connected: Vec<_> = find_not_enclosed(*last_position, &map, existing_path);
        // println!("Checking new position: {:?}, {:?} visited={:?} connected={:?}", last_position, map[last_position.0][last_position.1], visited, connected);
        let not_visited_connected: Vec<_> = connected.iter().cloned().filter(|e| !visited.contains(e)).collect();
        visited.insert(*last_position);
        if not_visited_connected.is_empty() {
            path.pop();
        } else {
            path.extend(not_visited_connected);
        }
    }
    visited.into_iter().collect()
}

fn find_chars(chs: &Vec<(usize, usize)>, map: &Vec<Vec<char>>) -> Vec<char> {
    chs.iter().map(|(x,y)| { map[*x][*y] }).collect()
}

fn find_connected_pipes((x, y): (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
    if x>0 && (map[x-1][y] == 'F' || map[x-1][y] == '|' || map[x-1][y] == '7') {
        result.push((x-1, y));
    }
    if y+1<map[0].len() && (map[x][y+1] == '-' || map[x][y+1] == 'J' || map[x][y+1] == '7') {
        result.push((x, y+1));
    }
    if x+1<map.len() && (map[x+1][y] == '|' || map[x+1][y] == 'J' || map[x+1][y] == 'L') {
        result.push((x+1, y));
    }
    if y>0 && (map[x][y-1] == '-' || map[x][y-1] == 'L' || map[x][y-1] == 'F') {
        result.push((x, y-1));
    }
    result
}

fn find_not_enclosed((x, y): (usize, usize), map: &Vec<Vec<char>>, existing_path: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
    if x+1<map.len() && !existing_path.contains(&(x+1, y)) {
        result.push((x+1, y));
    }
    if y+1<map[0].len() && !existing_path.contains(&(x, y+1)) {
        result.push((x, y+1));
    }
    if x>0 && !existing_path.contains(&(x-1, y)) {
        result.push((x-1, y));
    }
    if y>0 && !existing_path.contains(&(x, y-1)) {
        result.push((x, y-1));
    }
    result
}

fn find_start(ch: char, map: &Vec<Vec<char>>) -> (usize, usize) {
    for (row, line) in map.iter().enumerate() {
        for (column, char) in line.iter().enumerate() {
            if *char == ch {
                return (row, column)
            }
        }
    }
    panic!("Unable to find start")
}

// 713 high
// 65 wrong