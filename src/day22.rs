use std::cmp::min;
use std::collections::HashSet;
use std::fmt::format;

pub(crate) fn day22() {
    println!("Day 22");
    let file_content = include_str!("../resources/test-input.txt");
    // let file_content = include_str!("../resources/day22.txt").trim();
    println!("File content:\n{}", file_content);

    let blocks : Vec<_>= file_content.lines().map(|e| parse_line(e)).collect::<Vec<_>>();
    // println!("Blocks: {:?}", blocks);
    let (settled_blocks, counter): (Vec<_>, usize) = settle(&blocks);
    println!("Settled blocks: {:?}, counter {}", settled_blocks, counter);
    visualize_proj(&settled_blocks);

    part_1(&settled_blocks)
}

fn part_1(blocks: &Vec<Vec<[usize;3]>>) {
    let mut counter = 0;
    for index in 0..blocks.len() {
        let mut cloned_blocks = blocks.clone();
        cloned_blocks.remove(index);

        let (_, cnt) = settle(&cloned_blocks);
        if cnt == 0 {
            counter += 1;
        }
    }
    println!("{} bricks could be safely disintegrated", counter);
}

fn settle(blocks: &Vec<Vec<[usize;3]>>) -> (Vec<Vec<[usize;3]>>, usize) {
    let mut blocks_cloned = blocks.clone();
    blocks_cloned.sort_by(|e2, e1| {
       e1.iter().map(|v| v[2]).max().cmp(&e2.iter().map(|v| v[2]).max())
    });
    let mut result: Vec<Vec<[usize;3]>> = vec![];
    let mut total_counter = 0;
    while let Some(block) = blocks_cloned.pop() {
        // println!("Processing block: {:?}", block);
        let (settled_block, counter) = settle_block(block, &result);
        result.push(settled_block);
        total_counter += counter;
    }
    (result, total_counter)
}

fn settle_block(block: Vec<[usize;3]>, settled: &Vec<Vec<[usize;3]>>) -> (Vec<[usize;3]>, usize) {
    let mut cloned = block.clone();
    let mut counter: usize = 0;
    fn down(e: &Vec<[usize;3]>) -> Vec<[usize;3]> { e.iter().map(|e| [e[0], e[1], e[2]-1]).collect::<Vec<_>>() }
    while cloned.iter().all(|e| e[2] > 1) && !settled.iter().any(|s| block_collides(&down(&cloned), s)) {
        cloned = down(&cloned);
        counter += 1;
    }
    (cloned, counter)
}

fn block_collides(b1: &Vec<[usize;3]>, b2: &Vec<[usize;3]>) -> bool {
    let xz1_proj = b1.iter().map(|e| (e[0], e[2])).collect::<HashSet<(usize, usize)>>();
    let yz1_proj = b1.iter().map(|e| (e[1], e[2])).collect::<HashSet<(usize, usize)>>();

    let xz2_proj = b2.iter().map(|e| (e[0], e[2])).collect::<HashSet<(usize, usize)>>();
    let yz2_proj = b2.iter().map(|e| (e[1], e[2])).collect::<HashSet<(usize, usize)>>();

    xz1_proj.intersection(&xz2_proj).count() > 0 && yz1_proj.intersection(&yz2_proj).count() > 0
}

fn parse_line(line: &str) -> Vec<[usize;3]> {
    let parts: Vec<_> = line.split("~").collect::<Vec<_>>();
    let coords_start:Vec<_> = parts[0].split(",").map(|e| e.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let start: (usize, usize, usize) = (coords_start[0], coords_start[1], coords_start[2]);
    let coords_ned:Vec<_> = parts[1].split(",").map(|e| e.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let end: (usize, usize, usize) = (coords_ned[0], coords_ned[1], coords_ned[2]);
    // println!("start {:?}, end {:?}", start, end);
    let mut result: Vec<[usize;3]> = vec![];
    for x in start.0..=end.0 {
        for y in start.1..=end.1 {
            for z in start.2..=end.2 {
                result.push([x, y, z]);
            }
        }
    }
    result
}

fn visualize_proj(blocks: &Vec<Vec<[usize;3]>>) {
    let mut map_x: Vec<(usize, usize)> = vec![];
    let mut map_y: Vec<(usize, usize)> = vec![];
    for block in blocks {
        let xz_proj = block.iter().map(|e| (e[0], e[2])).collect::<Vec<(usize, usize)>>();
        map_x.extend(xz_proj);
        let yz_proj = block.iter().map(|e| (e[1], e[2])).collect::<Vec<(usize, usize)>>();
        map_y.extend(yz_proj);
    }
    let (max_x, _) = map_x.iter().max_by(|e1, e2| e1.0.cmp(&e2.0)).unwrap();
    let (max_y, _) = map_y.iter().max_by(|e1, e2| e1.0.cmp(&e2.0)).unwrap();
    let max_z = blocks.iter().filter_map(|e| e.iter().map(|e1| e1[2]).max()).max().unwrap();
    println!("==xz==");
    for z in (0..max_z).rev() {
        let mut str: String = String::from(format!("{} ", z));
        for x in 0..=*max_x {
            let counter = map_x.iter().filter(|e| *e == &(x, z)).count();
            if counter == 0 {
                str.push('.');
            } else if counter == 1 {
                str.push('#');
            } else {
                str.push('?');
            }
        }
        println!("{}", str);
    }
    println!("==yz==");
    for z in (0..max_z).rev() {
        let mut str: String = String::from(format!("{} ", z));
        for x in 0..=*max_y {
            let counter = map_y.iter().filter(|e| *e == &(x, z)).count();
            if counter == 0 {
                str.push('.');
            } else if counter == 1 {
                str.push('#');
            } else {
                str.push('?');
            }
        }
        println!("{}", str);
    }
}