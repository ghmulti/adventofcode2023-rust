use std::collections::HashSet;
use std::hash::Hash;
use std::time::Instant;

pub(crate) fn day21() {
    println!("Day 21");
    // let file_content = include_str!("../resources/test-input.txt");
    let file_content = include_str!("../resources/day21.txt").trim();
    // println!("File content:\n{}", file_content);

    // let map = format_map(file_content.lines().collect::<Vec<_>>());
    let map: Vec<_> = file_content.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    // println!("Map: {:?}", map);

    let start_vec = map.iter().enumerate().filter_map(|(row_index, row)| {
        row.iter().enumerate().find(|(_, &ch)| ch == 'S').map(|(column_index, _)| (row_index as isize, column_index as isize))
    }).collect::<Vec<_>>();
    let start = start_vec.first().unwrap();
    println!("Start point: {:?}", start);

    part_1(start.0 as usize, start.1 as usize, 64, &map);
    let now = Instant::now();
    // part_2(start, &map);
    part_2_v2(start, &map);
    println!("Part 2 took {}secs", now.elapsed().as_secs());
}

// https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
// https://www.youtube.com/watch?v=9UOMZSL0JTg&ab_channel=HyperNeutrino
fn part_2_v2(start: &(isize, isize), map: &Vec<Vec<char>>) {
    assert_eq!(map.len(), map[0].len());
    let steps = 26501365usize;
    let size = map.len();
    let sr = start.0 as usize;
    let sc = start.1 as usize;
    assert_eq!(sr, size/2);
    assert_eq!(sc, size/2);
    let grid_with = steps / size - 1;
    assert_eq!(grid_with, 202299);
    let odd_grids = (grid_with / 2 * 2 + 1).pow(2);
    let even_grids = ((grid_with + 1) / 2 * 2).pow(2);
    println!("odd grids {}, even grids {}", odd_grids, even_grids);
    let odd_points = part_2(sr, sc, size * 2 + 1, &map);
    let even_points = part_2(sr, sc, size * 2, &map);
    println!("odd points {} even points {}", odd_points, even_points);
    let corner_top = part_2(size - 1, sc, size  - 1, map);
    let corner_right = part_2(sr, 0, size  - 1, map);
    let corner_bottom = part_2(0, sc, size  - 1, map);
    let corner_left = part_2(sr, size - 1, size  - 1, map);

    let small_tr = part_2(size - 1, 0, size / 2 - 1, map);
    let small_tl = part_2(size - 1, size - 1, size / 2 - 1, map);
    let small_br = part_2(0, 0, size / 2 - 1, map);
    let small_bl = part_2(0, size - 1, size / 2 - 1, map);

    let large_tr = part_2(size - 1, 0, size * 3 / 2 - 1, map);
    let large_tl = part_2(size - 1, size - 1, size * 3 / 2 - 1, map);
    let large_br = part_2(0, 0, size * 3 / 2 - 1, map);
    let large_bl = part_2(0, size - 1, size * 3 / 2 - 1, map);

    let sm = (grid_with + 1) * (small_tr + small_tl + small_br + small_bl);
    let bg = grid_with * (large_tr + large_tl + large_br + large_bl);
    let crn = corner_top + corner_right + corner_bottom + corner_left;
    let result = (odd_grids * odd_points) + (even_grids * even_points) + crn + sm + bg;
    println!("Result: {}", result);
}

fn part_1(start_row: usize, start_column: usize, steps: usize, map: &Vec<Vec<char>>) -> usize {
    let mut positions: HashSet<(usize, usize, isize, isize)> = HashSet::new();
    positions.insert((start_row, start_column, 0, 0));
    // println!("{:?}", find_movement(*start, &map));
    for _ in 0..steps {
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
    positions.len()
}

fn part_2(start_row: usize, start_column: usize, steps: usize, map: &Vec<Vec<char>>) -> usize {
    let mut positions: HashSet<(usize, usize, isize, isize)> = HashSet::new();
    positions.insert((start_row, start_column, 0, 0));
    // println!("{:?}", find_movement(*start, &map));
    for index in 0..steps {
        let new_positions = positions.iter().fold(HashSet::new(), |mut acc, p| {
            let res = find_movement(*p, &map);
            acc.extend(res);
            acc
        });

        positions = new_positions;
    }
    // println!("Positions: {:?}", positions);
    println!("Number of positions for part 2={}", positions.len());
    // visualize(&positions, &map);
    positions.iter().filter(|&(_, _, tx, ty)| *tx == 0 && *ty == 0).collect::<Vec<_>>().len()
}

const N: (isize, isize) = (-1, 0);
const S: (isize, isize) = (1, 0);
const W: (isize, isize) = (0, -1);
const E: (isize, isize) = (0, 1);

fn find_movement(pos: (usize, usize, isize, isize), map: &Vec<Vec<char>>) -> Vec<(usize, usize, isize, isize)> {
    let positions = [N, S, W, E].iter()
        .map(|(x, y)| {
            (pos.0 as isize + x, pos.1 as isize + y, pos.2, pos.3)
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
            (new_x as usize, new_y as usize, new_tile_x, new_tile_y)
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