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

    part_1(&patterns);
    part_2(&patterns);
}

fn part_2(patterns: &Vec<Vec<String>>) {
    let notes: usize = patterns.iter().enumerate()
        .map(|(index, pattern)| {
            let res_v: usize = find_smudge(pattern);
            if res_v > 0 {
                return res_v * 100
            }
            let res_h: usize = find_smudge(&rotate(pattern));
            if res_h > 0 {
                return res_h
            }
            panic!("did not find solution for {}, pattern {:?}", index, pattern)
        })
        .sum::<usize>();
    println!("Summarizing of all notes: {}", notes)
}

fn find_smudge(patterns: &Vec<String>) -> usize {
    // println!("Processing patterns: {:?}", patterns);
    let default = find_v_reflection(&patterns, None);
    for (row_index, row) in patterns.iter().enumerate() {
        for (index, element)  in row.chars().enumerate() {
            let new_element = if element == '#' { '.' } else { '#' };
            let mut cloned_row: Vec<_> = row.chars().collect();
            cloned_row[index] = new_element;

            let mut cloned_pattern = patterns.clone();
            cloned_pattern[row_index] = cloned_row.iter().collect();

            let result = find_v_reflection(&cloned_pattern, Some(default));
            if result > 0 {
                // println!("Found smudge: {} {} for pattern {:?} with result {}", row_index, index, cloned_pattern, result);
                return result
            }
        }
    }
    return 0
}

fn part_1(patterns: &Vec<Vec<String>>) {
    let notes: usize = patterns.iter()
        .map(|pattern| {
            let v_reflection = find_v_reflection(pattern, None);
            let h_reflection = find_v_reflection(&rotate(pattern), None);
            (v_reflection * 100) + h_reflection
        })
        .sum::<usize>();
    println!("Summarizing of all notes: {}", notes)
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

fn find_v_reflection(pattern: &Vec<String>, index_to_ignore: Option<usize>) -> usize {
    let matching_rows: Vec<_> = pattern.windows(2).enumerate()
        .filter(|(_, w)| w[0] == w[1])
        .filter(|(index, _)| index_to_ignore.is_none() || index +1 != index_to_ignore.unwrap())
        .map(|(index, _)| (index as i16, index+1))
        .collect();

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
            return matching_row.1;
        }
    }

    0
}