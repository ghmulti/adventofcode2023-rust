use std::ops::RangeInclusive;

pub(crate) fn day22() {
    println!("Day 22");
    // let file_content = include_str!("../resources/test-input.txt");
    let file_content = include_str!("../resources/day22.txt").trim();
    // println!("File content:\n{}", file_content);

    let blocks : Vec<_>= file_content.lines().map(|e| parse_line(e)).collect::<Vec<_>>();
    // println!("Blocks: {:?}", blocks);
    let (settled_blocks, counter): (Vec<_>, Vec<usize>) = settle(&blocks);
    println!("Settled blocks counter {:?}", counter.iter().sum::<usize>());
    // visualize_proj(&settled_blocks);

    part_1(&settled_blocks);
    part_2(&settled_blocks);
}

fn part_2(blocks: &Vec<[RangeInclusive<usize>; 3]>) {
    let mut counter = 0;
    for index in 0..blocks.len() {
        let mut cloned_blocks = blocks.clone();
        cloned_blocks.remove(index);

        let (_, movement_counter) = settle(&cloned_blocks);
        counter += movement_counter.len();
        // println!("[{} of {}] Adding {}", index, blocks.len(), counter)
    }
    println!("{} blocks would fall in total", counter);
}

fn part_1(blocks: &Vec<[RangeInclusive<usize>; 3]>) {
    let mut counter = 0;
    for index in 0..blocks.len() {
        let mut cloned_blocks = blocks.clone();
        cloned_blocks.remove(index);

        let (_, movement_counter) = settle(&cloned_blocks);
        if movement_counter.len() == 0 {
            counter += 1;
        }
    }
    println!("{} bricks could be safely disintegrated", counter);
}

fn settle(blocks: &Vec<[RangeInclusive<usize>; 3]>) -> (Vec<[RangeInclusive<usize>; 3]>, Vec<usize>) {
    let mut blocks_cloned = blocks.clone();
    blocks_cloned.sort_by(|e1, e2| {
       e2[2].start().cmp(e1[2].start())
    });
    // println!("Sorted blocks: {:?}", blocks_cloned);
    let mut result: Vec<[RangeInclusive<usize>; 3]> = vec![];
    let mut total_block_movement_counter: Vec<usize> = vec![];
    while let Some(block) = blocks_cloned.pop() {
        // println!("Processing block: {:?}", block);
        let (settled_block, block_movement_counter) = settle_block(block, &result);
        result.push(settled_block);
        if block_movement_counter > 0 {
            total_block_movement_counter.push(block_movement_counter);
        }
    }
    (result, total_block_movement_counter)
}

fn settle_block(block: [RangeInclusive<usize>; 3], settled: &Vec<[RangeInclusive<usize>; 3]>) -> ([RangeInclusive<usize>; 3], usize) {
    let mut cloned = block.clone();
    let mut counter: usize = 0;
    fn down(e: &[RangeInclusive<usize>; 3]) -> [RangeInclusive<usize>; 3] {
        [e[0].clone(), e[1].clone(), (*e[2].start()-1)..=(*e[2].end()-1)]
    }
    while cloned[2].start() > &1 && !settled.iter().any(|s| block_collides(&down(&cloned), s)) {
        cloned = down(&cloned);
        counter += 1;
    }
    (cloned, counter)
}

fn block_collides(b1: &[RangeInclusive<usize>; 3], b2: &[RangeInclusive<usize>; 3]) -> bool {
    let z_inter = range_intersects(&b1[2], &b2[2]);
    if !z_inter {
        return false
    }
    let x_inter = range_intersects(&b1[0], &b2[0]);
    let y_inter = range_intersects(&b1[1], &b2[1]);
    if x_inter && y_inter {
        return true
    }
    return false
}

fn range_intersects(range1: &RangeInclusive<usize>, range2: &RangeInclusive<usize>) -> bool {
    let (start1, end1) = (*range1.start(), *range1.end());
    let (start2, end2) = (*range2.start(), *range2.end());
    if end1 < start2 || end2 < start1 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::day22::range_intersects;

    #[test]
    fn check_range_intersects() {
        assert_eq!(range_intersects(&(0..=5), &(6..=10)), false);
        assert_eq!(range_intersects(&(0..=7), &(6..=10)), true);
        assert_eq!(range_intersects(&(5..=10), &(0..=2)), false);
        assert_eq!(range_intersects(&(0..=10), &(2..=5)), true);
    }
}

fn parse_line(line: &str) -> [RangeInclusive<usize>; 3] {
    let parts: Vec<_> = line.split("~").collect::<Vec<_>>();
    let coords_start:Vec<_> = parts[0].split(",").map(|e| e.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let start: (usize, usize, usize) = (coords_start[0], coords_start[1], coords_start[2]);
    let coords_ned:Vec<_> = parts[1].split(",").map(|e| e.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let end: (usize, usize, usize) = (coords_ned[0], coords_ned[1], coords_ned[2]);
    // println!("start {:?}, end {:?}", start, end);
    [start.0..=end.0, start.1..=end.1, start.2..=end.2]
}

fn visualize_proj(blocks: &Vec<[RangeInclusive<usize>; 3]>) {
    let max_z = blocks.iter().map(|e| e[2].end()).max().unwrap();
    let max_x = blocks.iter().map(|e| e[0].end()).max().unwrap();
    let max_y = blocks.iter().map(|e| e[1].end()).max().unwrap();
    println!("==xz==");
    for z in (0..=*max_z).rev() {
        let mut str: String = String::from(format!("{} ", z));
        for x in 0..=*max_x {
            let counter = blocks.iter().filter(|r|  r[2].contains(&z) && r[0].contains(&x)).count();
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
    for z in (0..=*max_z).rev() {
        let mut str: String = String::from(format!("{} ", z));
        for x in 0..=*max_y {
            let counter = blocks.iter().filter(|r|  r[2].contains(&z) && r[1].contains(&x)).count();
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