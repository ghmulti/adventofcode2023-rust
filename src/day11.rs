use std::cmp::min;

pub(crate) fn day11() {
    println!("Day 11");
    // let file_content = include_str!("../resources/test-input.txt");
    let file_content = include_str!("../resources/day11.txt");
    // println!("File content:\n{}", file_content);
    let lines: Vec<_> = file_content.lines().collect();
    let mut map: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    expand(&mut map);
    // println!("Map:\n{}", map.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));

    let galaxies: Vec<_> = find_galaxies(&map);
    // println!("Galaxies: {:?}", galaxies);
    let pairs: Vec<_> = find_pairs(&galaxies);

    part_1(&pairs, &map, 2);
    part_1(&pairs, &map, 1000000);
}

fn part_1(pairs: &Vec<((usize, usize), (usize, usize))>, map: &Vec<Vec<char>>, multiplier: usize) {
    let distances = pairs.iter().map(|(p1, p2)| {
        find_distance(p1, p2, multiplier, map)
    });
    println!("Sum of distances: {}", distances.sum::<usize>());
}

fn find_pairs(galaxies: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    let mut pairs: Vec<((usize, usize), (usize, usize))> = vec![];
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            pairs.push((galaxies[i].clone(), galaxies[j].clone()));
        }
    }
    // println!("Unique pairs: {:?} [total {}]", pairs, pairs.len());
    pairs
}

fn find_distance((x1, y1): &(usize, usize), (x2, y2): &(usize, usize), multiplier: usize, map: &Vec<Vec<char>>) -> usize {
    let x_diff = x2.abs_diff(*x1);
    let y_diff = y2.abs_diff(*y1);
    let min_x = min(*x2, *x1);
    let min_y = min(*y2, *y1);
    let x_dist: usize = (min_x..(min_x+x_diff)).map(|x| {
        if map[x][min_y] == 'o' { multiplier } else { 1 }
    }).sum();
    let y_dist: usize = (min_y..(min_y+y_diff)).map(|y| {
        if map[min_x][y] == 'o' { multiplier } else { 1 }
    }).sum();
    let distance = x_dist + y_dist;
    // println!("Distance for [{},{}] [{},{}] = {}", x1, y1, x2, y2, distance);
    distance
}

fn find_galaxies(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
    for (row, line) in map.iter().enumerate() {
        for (column, char) in line.iter().enumerate() {
            if *char == '#' {
                result.push((row, column))
            }
        }
    }
    result
}

fn expand(map: &mut Vec<Vec<char>>) {
    let empty_rows: Vec<_> = map.iter().enumerate().filter(|(_, row)| {
        row.iter().all(|ch| *ch == '.')
    }).map(|(index, _)| index).collect();
    // println!("Empty rows: {:?}", empty_rows);

    let empty_columns: Vec<_> = (0..map.len()).filter(|index| {
        map.iter().map(|row| row[*index]).all(|ch| ch == '.')
    }).collect();
    // println!("Empty columns: {:?}", empty_columns);

    for e in (0..map.len()).rev() {
        if empty_rows.contains(&e) {
            map[e] = map[e].clone().iter().map(|_| 'o').collect();
        }
    }

    for e in (0..map[0].len()).rev() {
        if empty_columns.contains(&e) {
            for line in map.iter_mut() {
                line[e] = 'o'
            }
        }
    }
}
