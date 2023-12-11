use std::iter::zip;

pub(crate) fn day11() {
    println!("Day 11");
    // let file_content = include_str!("../resources/test-input.txt");
    let file_content = include_str!("../resources/day11.txt");
    // println!("File content: {}", file_content);
    let lines: Vec<_> = file_content.lines().collect();
    let mut map: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    expand(&mut map);
    // println!("Map:\n{}", map.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));

    let galaxies: Vec<_> = find_galaxies(&map);
    // println!("Galaxies: {:?}", galaxies);

    part_1(&galaxies, &map);
}

fn part_1(galaxies: &Vec<(usize, usize)>, map: &Vec<Vec<char>>) {
    let mut pairs: Vec<((usize, usize), (usize, usize))> = vec![];
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            pairs.push((galaxies[i].clone(), galaxies[j].clone()));
        }
    }
    // println!("Unique pairs: {:?} [total {}]", pairs, pairs.len());

    let distances = pairs.iter().map(|(p1, p2)| {
        find_distance(p1, p2)
    });
    println!("Sum of distances: {}", distances.sum::<usize>());
}

fn find_distance((x1, y1): &(usize, usize), (x2, y2): &(usize, usize)) -> usize {
    let x_diff = x2.abs_diff(*x1);
    let y_diff = y2.abs_diff(*y1);
    let distance = x_diff + y_diff;
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
            map.insert(e, map[e].clone())
        }
    }

    for e in (0..map[0].len()).rev() {
        if empty_columns.contains(&e) {
            for line in map.iter_mut() {
                line.insert(e, '.')
            }
        }
    }
}
