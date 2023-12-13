use std::collections::HashMap;

pub(crate) fn day13() {
    println!("Day 13");
    // let file_content = include_str!("../resources/test-input.txt");
    let file_content = include_str!("../resources/day13.txt");
    // println!("File content:\n{}", file_content);
    let lines: Vec<_> = file_content.lines().collect();
    let lines_parsed: Vec<_> = lines.split(|line| line.is_empty()).collect();
    let patterns: Vec<Vec<String>> = lines_parsed.iter()
        .map(|&str_slice| str_slice.iter().map(|&s| s.to_string()).collect())
        .collect();
    // println!("Patterns: {:?}", patterns);

    let number_of_rows: usize = patterns.iter()
        .map(|e| find_v_reflection(e) * 100)
        .sum();
    let number_of_columns: usize = patterns.iter()
        .map(|pattern| rotate(pattern))
        .map(|pattern| find_v_reflection(&pattern))
        .sum();
    println!("Summarizing of all notes: {}", number_of_rows+number_of_columns)
}

fn rotate(pattern: &Vec<String>) -> Vec<String> {
    (0..pattern[0].len()).map(|column| {
        let chars: Vec<_> = (0..pattern.len()).rev().map(|row| {
            pattern[row].chars().nth(column).unwrap()
        }).collect();
        let str = chars.iter().collect();
        str
    }).collect()
}

fn find_v_reflection(pattern: &Vec<String>) -> usize {
    // println!("Working with pattern: {:?}", pattern);
    let matching_rows: Vec<_> = pattern.windows(2).enumerate()
        .filter(|(_, w)| w[0] == w[1])
        .map(|(index, _)| (index as i16, index+1))
        .collect();

    let mut res: Vec<usize> = vec![];
    // println!("Matching rows: {:?}", matching_rows);
    for matching_row in matching_rows {
        let (mut i1, mut i2) = matching_row.clone();
        while i1 >= 0 && i2 < pattern.len() {
            if pattern[i1 as usize] == pattern[i2] {
                i1 -= 1;
                i2 += 1;
            } else {
                break
            }
        }
        if i1 == -1 || i2 == pattern.len() {
            res.push(matching_row.1);
        }
    }
    // println!("Reflection rows: {:?}", res);
    // println!("Reflection rows length: {:?}", res.len());
    res.iter().sum::<usize>()
}

