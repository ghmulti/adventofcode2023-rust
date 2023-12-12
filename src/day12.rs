pub(crate) fn day12() {
    println!("Day 12");
    let file_content = include_str!("../resources/test-input.txt");
    // let file_content = include_str!("../resources/day12.txt");
    // println!("File content:\n{}", file_content);
    let lines: Vec<_> = file_content.lines().collect();

    part_1(&lines);
    // part_2(&lines);
}

fn parse_line(line: &str) -> (Vec<usize>, &str){
    let parts: Vec<_> = line.split_whitespace().collect();
    let group: Vec<usize> = parts[1].split(",").map(|e| e.parse::<usize>().unwrap()).collect();
    let template = parts[0];
    (group, template)
}

fn part_1(lines: &Vec<&str>) {
    let combinations: Vec<_> = lines.iter().map(|line| {
        let (mut group, template) = parse_line(line);
        let combinations_length = find_combinations(&mut template.chars().collect(), &mut group, 0);
        println!("Found {} combinations for {:?}", combinations_length, template);
        combinations_length
    }).collect();
    println!("Sum of combinations length: {:?}", combinations.iter().sum::<i64>());
}

fn part_2(lines: &Vec<&str>) {
    let combinations: Vec<_> = lines.iter().map(|line| {
        let (group_1, template_1) = parse_line(line);
        let mut template_2: Vec<_>  = (0..5).map(|e| template_1).collect::<Vec<_>>().join("?").chars().collect();
        let mut group_2 = group_1.repeat(5);
        find_combinations(&mut template_2, &mut group_2, 0)
    }).collect();
    println!("Sum of combinations length: {:?}", combinations.iter().sum::<i64>());
}
fn find_combinations(springs: &mut Vec<char>, groups: &Vec<usize>, counter: usize) -> i64 {
    // println!("Working with springs {:?} groups {:?} counter {}", springs, groups, counter);
    if springs.is_empty() {
        if groups.len() == 1 && counter == groups[0] || groups.is_empty() && counter == 0 {
            return 1
        }
        return 0
    }
    let spring = springs[0];
    springs.remove(0);
    let number_in_group = if groups.is_empty() { 0 } else { groups[0] };
    if spring == '?' {
        let mut p1 = vec!['#'];
        let mut p2 = vec!['.'];
        p1.extend(springs.clone());
        p2.extend(springs.clone());
        // println!("Searching for {:?}", p1);
        let p1_len = find_combinations(&mut p1, groups, counter);
        // println!("Searching for {:?}", p2);
        let p2_len = find_combinations(&mut p2, groups, counter);
        // println!("Combining results {} {}", p1_len, p2_len);
        return p1_len + p2_len
    }
    if spring == '#' {
        return if counter > number_in_group {
            0
        } else {
            find_combinations(springs, groups, counter + 1)
        }
    }
    if spring == '.' {
        return if counter == 0 {
            find_combinations(springs, groups, 0)
        } else if counter == number_in_group {
            let mut new_groups = groups.clone();
            new_groups.remove(0);
            find_combinations(springs, &new_groups, 0)
        } else {
            0
        }
    }
    panic!("Invalid spring {}", spring)
}