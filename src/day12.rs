use std::fmt::format;

pub(crate) fn day12() {
    println!("Day 12");
    let file_content = include_str!("../resources/test-input.txt");
    // let file_content = include_str!("../resources/day12.txt");
    // println!("File content:\n{}", file_content);
    let lines: Vec<_> = file_content.lines().collect();

    // part_1(&lines);
    part_2(&lines);

}

fn parse_line(line: &str) -> (Vec<usize>, &str){
    let parts: Vec<_> = line.split_whitespace().collect();
    let group: Vec<usize> = parts[1].split(",").map(|e| e.parse::<usize>().unwrap()).collect();
    let template = parts[0];
    (group, template)
}

fn part_1(lines: &Vec<&str>) {
    let combinations: Vec<Vec<String>> = lines.iter().map(|line| {
        let (group, template) = parse_line(line);
        let combinations = find_combinations(template, &group);
        // println!("Combinations for template {:?} with groups {:?}: {:?}", template, group, combinations);
        combinations
    }).collect();
    let combination_length: Vec<usize> = combinations.iter().map(|e| e.len()).collect();
    println!("Number of combinations: {:?}", combination_length);
    println!("Sum of combinations length: {:?}", combination_length.iter().sum::<usize>());
}

fn part_2(lines: &Vec<&str>) {
    let combinations: Vec<i64> = lines.iter().map(|line| {
        let (group_1, template_1) = parse_line(line);
        let combinations1 = find_combinations(template_1, &group_1);
        let template_2: String  = (0..5).map(|e| template_1).collect::<Vec<_>>().join("?");
        let group_2 = group_1.repeat(5);
        let combinations2 = find_combinations(&template_2, &group_2);
        println!("Combinations {:?} [{}], {:?} [{}]", template_1, combinations1.len(), template_2, 0);
        0
    }).collect();
    println!("Number of combinations: {:?}", combinations);
    println!("Sum of combinations length: {:?}", combinations.iter().sum::<i64>());
}
fn find_combinations(template: &str, groups: &Vec<usize>) -> Vec<String> {
    let unknowns: Vec<usize> = template.chars().enumerate().filter(|(_, ch)| *ch == '?').map(|(index, _)| index).collect();
    // println!("Searching for combinations for {:?}, group {:?}, unknown indexes {:?}", template, groups, unknowns);
    let possible_combinations = generate_combinations(unknowns.len(), &vec!['.', '#'], &mut Vec::new());
    // println!("Possible combinations {:?} [{}]", possible_combinations, possible_combinations.len());
    let mut result: Vec<String> = vec![];
    for combination in possible_combinations {
        let mut value: Vec<char> = template.chars().clone().collect();
        for (char, index) in combination.iter().zip(unknowns.iter()) {
            value[*index] = *char
        }
        if check_value(&value, groups) {
            result.push(value.iter().collect())
        }
    }
    // println!("Working combinations {:?} [{}]", result, result.len());
    result
}

fn check_value(value: &Vec<char>, groups: &Vec<usize>) -> bool {
    // println!("Checking value {:?} against group {:?}", value, groups);
    calculate_consecutive_elements(value) == *groups
}

fn calculate_consecutive_elements(value: &Vec<char>) -> Vec<usize> {
    let mut result: (usize, Vec<usize>) = value.iter().fold( (0, vec![]), |mut acc, e| {
        if *e == '#' {
            acc.0 += 1;
            acc
        } else if acc.0 > 0 {
            (acc.1).push(acc.0);
            (0, acc.1)
        } else {
            acc
        }
    });
    if result.0 > 0 {
        result.1.push(result.0);
    }
    result.1
}

fn generate_combinations(length: usize, chars: &Vec<char>, acc: &mut Vec<char>) -> Vec<Vec<char>> {
    if acc.len() == length {
        return vec![acc.clone()];
    }
    let mut result : Vec<Vec<char>> = vec![];
    for &ch in chars.iter() {
        acc.push(ch);
        let nested = generate_combinations(length, chars, acc);
        result.extend(nested);
        acc.pop();
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::day12::calculate_consecutive_elements;

    #[test]
    fn check() {
        assert_eq!(calculate_consecutive_elements(&vec!['.', '#', '#', '#', '.', '.', '.', '#', '#', '.', '.', '#']), [3, 2, 1]);
        assert_eq!(calculate_consecutive_elements(&vec!['.', '#', '#', '#', '.', '#', '.', '#', '#', '.', '.', '#']), [3, 1, 2, 1]);
    }
}