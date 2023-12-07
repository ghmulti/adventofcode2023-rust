use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use lazy_static::lazy_static;

use crate::day07::HandType::{Five, Four, FullHouse, High, OnePair, Three, TwoPair};

lazy_static! {
    static ref HAND_TYPE_VALUE_1: HashMap<char, u8> = {
        let mut hand_type_value = HashMap::new();
        hand_type_value.insert('A', 14);
        hand_type_value.insert('K', 13);
        hand_type_value.insert('Q', 12);
        hand_type_value.insert('J', 11);
        hand_type_value.insert('T', 10);
        hand_type_value.insert('9', 9);
        hand_type_value.insert('8', 8);
        hand_type_value.insert('7', 7);
        hand_type_value.insert('6', 6);
        hand_type_value.insert('5', 5);
        hand_type_value.insert('4', 4);
        hand_type_value.insert('3', 3);
        hand_type_value.insert('2', 2);
        hand_type_value
    };

    static ref HAND_TYPE_VALUE_2: HashMap<char, u8> = {
        let mut hand_type_value = HashMap::new();
        hand_type_value.insert('A', 14);
        hand_type_value.insert('K', 13);
        hand_type_value.insert('Q', 12);
        hand_type_value.insert('T', 10);
        hand_type_value.insert('9', 9);
        hand_type_value.insert('8', 8);
        hand_type_value.insert('7', 7);
        hand_type_value.insert('6', 6);
        hand_type_value.insert('5', 5);
        hand_type_value.insert('4', 4);
        hand_type_value.insert('3', 3);
        hand_type_value.insert('2', 2);
        hand_type_value.insert('J', 1);
        hand_type_value
    };
}

pub(crate) fn day7() {
    println!("Day 7");
    let file_path = "resources/day07.txt";
    // let file_path = "resources/test-input.txt";
    let mut file = File::open(file_path).expect("Unable to open file");

    let mut buffer = String::new();
    file.read_to_string( &mut buffer).expect("Unable to read file into string");
    println!("File contents:\n{}", buffer);

    let lines: Vec<String> = buffer.lines().map(String::from).collect();

    let parsed_hands_1: Vec<_> = lines.iter().map(|line| {
        let parts: Vec<_> = line.split_whitespace().collect();
        Hand {
            hand_type: parse_hand_type(&parts[0]),
            value: String::from(parts[0]),
            bid: parts[1].parse::<u16>().expect("Invalid bid number")
        }
    }).collect();
    // println!("Parsed hands: {:?}", parsed_hands_1);
    part_1(parsed_hands_1, &HAND_TYPE_VALUE_1);

    let parsed_hands_2: Vec<_> = lines.iter().map(|line| {
        let parts: Vec<_> = line.split_whitespace().collect();
        let combinations = build_joker_combinations(&parts[0]);
        // println!("Joker combinations: {:?}", combinations);
        let best_hand_type: HandType = combinations.iter().fold(parse_hand_type(&*combinations[0]), |current_best, hand| {
            let possible_hand_type = parse_hand_type(hand);
            if possible_hand_type < current_best { possible_hand_type } else { current_best }
        });
        println!("Best hand type for {}: {:?}", &parts[0], best_hand_type);
        Hand {
            hand_type: best_hand_type,
            value: String::from(parts[0]),
            bid: parts[1].parse::<u16>().expect("Invalid bid number")
        }
    }).collect();
    part_1(parsed_hands_2, &HAND_TYPE_VALUE_2); // 248029057
}

fn part_1(mut cloned_hands: Vec<Hand>, value_mapper: &HashMap<char, u8>) {
    cloned_hands.sort_by(|hand1, hand2| {
        if hand1.hand_type != hand2.hand_type {
            hand2.hand_type.cmp(&hand1.hand_type)
        } else {
            let hand1_values = hand1.value.chars().map(|e| value_mapper[&e]);
            let hand2_values = hand2.value.chars().map(|e| value_mapper[&e]);
            hand1_values.cmp(hand2_values)
        }
    });
    println!("Sorted hands: {:?}", cloned_hands);
    let bids : Vec<_> = cloned_hands.iter().enumerate().map(|(index, hand)| {
        (index+1) * hand.bid as usize
    }).collect();
    println!("Bids: {:?}", bids);
    println!("Total winnings: {}", bids.iter().sum::<usize>())
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    Five, Four, FullHouse, Three, TwoPair, OnePair, High, Unknown
}

#[derive(Debug, Clone, PartialEq)]
struct Hand {
    hand_type: HandType,
    value: String,
    bid: u16,
}

fn build_joker_combinations(hand: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![String::from(hand)];
    let hand_chars: Vec<char> = hand.chars().collect();
    let joker_indexes: Vec<_> = hand.char_indices().filter(|&(_, c)| c == 'J').map(|(index, _)| index).collect();
    for index in joker_indexes {
        for ch in ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'] {
            let mut hand_chars_cloned: Vec<_> = hand_chars.clone();
            hand_chars_cloned[index] = ch;
            let mut options: Vec<String> = build_joker_combinations(&hand_chars_cloned.into_iter().collect::<String>());
            result.append(&mut options)
        }
    }
    return result;
}

fn generate_combinations(s: &str, current: String, index: usize) {
    if index == s.len() {
        // We reached the end of the string, print the current combination
        println!("{}", current);
    } else {
        // If the current character is 'a', branch into both possibilities ('c' and 'd')
        if s.chars().nth(index) == Some('a') {
            generate_combinations(s, current.clone() + "c", index + 1);
            generate_combinations(s, current + "d", index + 1);
        } else {
            // If the current character is not 'a', keep it unchanged
            generate_combinations(s, current + &s[index..index + 1], index + 1);
        }
    }
}

fn parse_hand_type(hand: &str) -> HandType {
    let mut char_count: HashMap<char, u8> = HashMap::new();
    for c in hand.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    }
    let mut sorted_char_count: Vec<_> = char_count.into_iter().collect();
    sorted_char_count.sort_by(|(_, cnt1), (_, cnt2)| { cnt2.cmp(cnt1) });
    // println!("Sorted char count for {}: {:?}", line, sorted_char_count);
    let (_, top_cnt) = *sorted_char_count.first().expect("Unable to get first");
    let hand_type: HandType = match top_cnt {
        5 => Five,
        4 => Four,
        3 => {
            if sorted_char_count[0].1 == 3 && sorted_char_count[1].1 == 2 {
                FullHouse
            } else {
                Three
            }
        },
        2 => {
            if sorted_char_count[0].1 == 2 && sorted_char_count[1].1 == 2 {
                TwoPair
            } else {
                OnePair
            }
        },
        1 => High,
        _ => panic!("Unknown hand type")
    };
    hand_type
}

#[cfg(test)]
mod tests {
    use crate::day07::{Hand, HandType, parse_hand_type};

    #[test]
    fn check() {
        assert_eq!(parse_hand_type("32T3K"), HandType::OnePair);
        assert_eq!(parse_hand_type("3223K"), HandType::TwoPair);
        assert_eq!(parse_hand_type("12345"), HandType::High);
        assert_eq!(parse_hand_type("11145"), HandType::Three);
        assert_eq!(parse_hand_type("11111"), HandType::Five);
        assert_eq!(parse_hand_type("A1111"), HandType::Four);
        assert_eq!(parse_hand_type("AA111"), HandType::FullHouse);
    }
}