use std::fs::File;
use std::io::Read;

pub(crate) fn day1() {
    println!("Day 1");
    let file_path = "resources/day01.txt";
    // let file_path = "resources/test-input.txt";
    let mut file = File::open(file_path).expect("Unable to open file");

    let mut buffer = String::new();
    file.read_to_string( &mut buffer).expect("Unable to read file into string");
    //println!("File contents:\n{}", buffer);

    let lines: Vec<String> = buffer.lines().map(String::from).collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let line_numbers: Vec<u32> = lines.iter()
        .fold(Vec::new(), |mut acc, line| {
            let nums = line.chars().fold(Vec::new(), |mut acc, c| {
                if c.is_digit(10) {
                    acc.push(c.to_digit(10).unwrap())
                }
                acc
            });
            let first = nums.first().expect("missing number");
            let last = nums.last().expect("missing number");

            let number = format!("{}{}", first, last);
            acc.push(number.parse::<u32>().unwrap());
            acc
        });

    let numbers_sum: u32 = line_numbers.iter().sum();
    println!("Sum of numbers: {}", numbers_sum);
}

fn part2(lines: &Vec<String>) {
    let convert_map = [
        ("oneight", "18"),
        ("threeight", "38"),
        ("fiveight", "58"),
        ("sevenine", "79"),
        ("eightwo", "82"),
        ("eighthree", "83"),
        ("nineight", "98"),
        ("twone", "21"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let converted_lines: Vec<String> = lines.iter().map(|line| {
        // let result = line.chars().fold(String::new(), |mut acc, ch| {
        //     acc.push(ch);
        //     let res = convert_map.iter().fold(acc, |substr, (key, value)| {
        //         substr.replace(key, value)
        //     });
        //     res
        // });
        let result = convert_map.iter().fold(line.to_string(), |acc, (key, value)| {
            acc.replace(key, value)
        });
        // println!("From {} to {}", line, result);
        result
    }).collect();

    part1(&converted_lines);
}