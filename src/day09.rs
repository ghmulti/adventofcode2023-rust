use std::fs::File;
use std::io::Read;

pub(crate) fn day9() {
    println!("Day 9");
    let file_path = "resources/day09.txt";
    // let file_path = "resources/test-input.txt";
    let mut file = File::open(file_path).expect("Unable to open file");

    let mut buffer = String::new();
    file.read_to_string( &mut buffer).expect("Unable to read file into string");
    // println!("File contents:\n{}", buffer);

    let lines: Vec<String> = buffer.lines().map(String::from).collect();
    let report_entries: Vec<_> = lines.iter().map(|e| {
        e.split_whitespace().map(| t| t.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>()
    }).collect();
    // println!("Report entries: {:?}", report_entries);

    part_1(&report_entries);
    part_2(&report_entries);
}

fn part_2(report_entries: &Vec<Vec<i64>>) {
    let reversed_entries: Vec<_> = report_entries.iter().map(|e| {
        e.iter().cloned().rev().collect()
    }).collect();
    // println!("Reversed entries: {:?}", reversed_entries);
    part_1(&reversed_entries)
}

fn part_1(report_entries: &Vec<Vec<i64>>) {
    let extrapolated_values: Vec<_> = report_entries.iter().map(|report_entry| {
        let processed_records: Vec<_> = process(report_entry);
        let extrapolate: Vec<_> = extrapolate(processed_records);
        let extrapolated_value: &i64 = extrapolate.last().unwrap().last().unwrap();
        *extrapolated_value
    }).collect();
    // println!("Extrapolated values: {:?}", extrapolated_values);
    println!("Sum of extrapolated values: {:?}", extrapolated_values.iter().sum::<i64>())
}

fn process(record: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut result: Vec<Vec<i64>> = vec![record.clone()];
    while result.last().unwrap().iter().any(|e| *e != 0) {
        let next_line: Vec<_> = result.last().unwrap().windows(2).map(|e| e[1] - e[0]).collect();
        result.push(next_line);
    }
    // println!("Processed records: {:?}", result);
    result
}

fn extrapolate(processed_records: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut result: Vec<Vec<i64>> = vec![];
    let mut modifier: i64 = 0;
    for e in processed_records.iter().rev() {
        let mut cloned = e.clone();
        cloned.push(cloned.last().unwrap() + modifier);
        result.push(cloned);
        modifier = e.last().unwrap() + modifier;
    }
    // println!("Extrapolated records: {:?}", result);
    result
}