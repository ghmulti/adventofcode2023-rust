use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;

pub(crate) fn day2() {
    println!("Day 2");
    let file_path = "resources/day02.txt";
    // let file_path = "resources/test-input.txt";
    let mut file = File::open(file_path).expect("Unable to open file");

    let mut buffer = String::new();
    file.read_to_string( &mut buffer).expect("Unable to read file into string");
    // println!("File contents:\n{}", buffer);

    let lines: Vec<String> = buffer.lines().map(String::from).collect();
    let rounds: Vec<Round> = lines.iter().map(|l| { parse_round(l) }).collect();

    part_1(&rounds);
    part_2(&rounds);
}

struct Round {
    index: usize,
    sets: Vec<RGB>,
}

struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

fn parse_round(line: &String) -> Round {
    let game_and_sets: Vec<&str> = line.split(":").collect();
    let game_index = game_and_sets[0].replace("Game ", "").parse::<usize>().expect("Invalid round index");
    let set_of_cubes: Vec<&str> = game_and_sets[1].split(";").collect();

    let game_sets: Vec<RGB> = set_of_cubes.iter().fold(Vec::new(), |mut acc, current_set| {
        let rgb = current_set.split(",").fold(RGB { blue: 0, red: 0, green: 0}, |mut rgb, set_entry| {
            if set_entry.contains("green") {
                let green_score = set_entry.replace(" green", "").trim().parse::<u8>().expect("Invalid green score");
                rgb.green += green_score;
            } else if set_entry.contains("blue") {
                let blue_score = set_entry.replace(" blue", "").trim().parse::<u8>().expect("Invalid green score");
                rgb.blue += blue_score;
            } else if set_entry.contains("red") {
                let red_score = set_entry.replace(" red", "").trim().parse::<u8>().expect("Invalid green score");
                rgb.red += red_score;
            }
            rgb
        });
        acc.push(rgb);
        acc
    });

    let round = Round {
        sets: game_sets,
        index: game_index,
    };
    // println!("{}", round);

    round
}

fn part_1(rounds: &Vec<Round>) {
    let result: u16 = rounds.iter().filter(|round| {
        let possible = round.sets.iter().all(|set| {
            set.red <= 12 && set.green <= 13 && set.blue <= 14
        });
        // println!("Round {}, possible {}", round, possible);
        possible
    }).map(|round| { round.index as u16 }).sum();
    println!("Sum of IDs: {}", result);
}

fn part_2(rounds: &Vec<Round>) {
    let result: u64 = rounds.iter().map(|round| {
        let min_green = round.sets.iter().map(| set| { set.green }).filter(|x| x > &0).max().expect("Green not defined") as u64;
        let min_red = round.sets.iter().map(| set| { set.red }).filter(|x| x > &0).max().expect("Red not defined") as u64;
        let min_blue = round.sets.iter().map(| set| { set.blue }).filter(|x| x > &0).max().expect("Blue not defined") as u64;
        // println!("Round {}, min_green {}, min_red {}, min_blue {}", round, min_green, min_red, min_blue);
        min_green * min_blue * min_red
    }).sum();
    println!("Sum of power of sets: {}", result);
}

impl Display for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "red {}, green: {}, blue: {}", self.red, self.green, self.blue)
    }
}

impl Display for Round {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Round {{ index: {}, sets: [", self.index)?;
        for rgb in self.sets.iter() {
            write!(f, "Set {{ {} }}, ", rgb)?;
        }
        write!(f, "] }}")
    }
}