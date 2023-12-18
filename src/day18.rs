const R: (isize, isize) = (0, 1);
const D: (isize, isize) = (1, 0);
const L: (isize, isize) = (0, -1);
const U: (isize, isize) = (-1, 0);

pub(crate) fn day18() {
    println!("Day 18");
    // let file_content = include_str!("../resources/test-input.txt");
    let file_content = include_str!("../resources/day18.txt").trim();
    // println!("File content:\n{}", file_content);

    let lines: Vec<_> = file_content.lines().map(|line| parse_line(line)).collect();
    let instructions_1: Vec<_> = lines.iter().map(|line| line.0).collect::<Vec<_>>();
    part_1(&instructions_1);
    let instructions_2: Vec<_> = lines.iter().map(|line| line.1).collect::<Vec<_>>();
    part_1(&instructions_2);
}

fn part_1(lines: &Vec<((isize, isize), isize)>) {
    let start: (isize, isize) = (0, 0);
    let mut path: Vec<(isize, isize)> = vec![start];
    for line in lines {
        // println!("Line: {:?}", line);
        for _ in 0..line.1 {
            let (x, y) = *path.last().unwrap();
            let current: (isize, isize) = (x + line.0.0, y + line.0.1);
            path.push(current);
        }
    }
    // println!("Path: {:?}", path);

    let y_offset = path.iter().map(|(_, y)| *y).max().unwrap();
    let inside_area = calculate_area(&path, y_offset);
    println!("Area: {:?}", inside_area + path.len() - 1);
}

fn parse_line(line: &str) -> (((isize, isize), isize), ((isize, isize), isize)) {
    // println!("Parsing line: {:?}", line);
    let parts: Vec<_> = line.split_whitespace().collect();
    let dir = match parts[0] {
        "R" => R,
        "L" => L,
        "D" => D,
        "U" => U,
        &_ => panic!("🤯")
    };
    let multiplier = parts[1].parse::<isize>().unwrap();
    let hex_distance = &parts[2][2..7];
    let hex_dir = parts[2].chars().nth(7);
    let hex_1 = u32::from_str_radix(hex_distance, 16).unwrap() as isize;
    let hex_2 = match hex_dir.unwrap() {
        '0' => R,
        '1' => D,
        '2' => L,
        '3' => U,
        _ => panic!("🤯")
    };
    (
        ((dir.0, dir.1), multiplier),
        ((hex_2.0, hex_2.1), hex_1)
    )
}


fn calculate_area(path: &Vec<(isize, isize)>, y_offset: isize) -> usize {
    let polygon_area = path
        .windows(2)
        .map(|w| {
            // https://en.wikipedia.org/wiki/Shoelace_formula
            let (p1, p2) = (w[0], w[1]);
            let x1 = p1.0 as i64;
            let y1 = p1.1 as i64;
            let x2 = p2.0 as i64;
            let y2 = p2.1 as i64;
            // cartesian coordinate system is required, adjusting y
            let offset = y_offset as i64;
            let y1 = offset - y1;
            let y2 = offset - y2;
            (x1 * y2) - (x2 * y1)
        }).sum::<i64>() as usize / 2;

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    // A = i + b / 2 - 1   ====>   i = A - b / 2 + 1
    let number_of_interior_points = polygon_area - path.len() / 2 + 1;
    number_of_interior_points
}