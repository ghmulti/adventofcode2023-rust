use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::day07::HandType::{Five, Four, FullHouse, High, OnePair, Three, TwoPair};

pub(crate) fn day7() {
    println!("Day 7");
    let file_path = "resources/day07.txt";
    // let file_path = "resources/test-input.txt";
    let mut file = File::open(file_path).expect("Unable to open file");

    let mut buffer = String::new();
    file.read_to_string( &mut buffer).expect("Unable to read file into string");
    // println!("File contents:\n{}", buffer);

    let lines: Vec<String> = buffer.lines().map(String::from).collect();

    part_1(&lines);
    part_2(&lines);
}

fn part_1(lines: &Vec<String>) {
    let mut parsed_hands_1: Vec<_> = lines.iter().map(|line| {
        let parts: Vec<_> = line.split_whitespace().collect();
        Hand {
            hand_type: parse_hand_type(&parts[0]),
            value: parts[0],
            bid: parts[1].parse::<u16>().expect("Invalid bid number")
        }
    }).collect();
    // println!("Parsed hands: {:?}", parsed_hands_1);
    let value_mapper_1 : HashMap<char, u8> = HashMap::from(
        [('A', 14),('K', 13),('Q', 12),('J', 11),('T', 10),('9', 9),('8', 8),('7', 7),('6', 6),('5', 5),('4', 4),('3', 3),('2', 2),]
    );
    calculate_total_winnings(&mut parsed_hands_1, &value_mapper_1);
}

fn part_2(lines: &Vec<String>) {
    let mut parsed_hands_2: Vec<_> = lines.iter().map(|line| {
        let parts: Vec<_> = line.split_whitespace().collect();
        let combinations = build_joker_combinations(&parts[0]);
        // println!("Joker combinations: {:?}", combinations);
        let best_hand_type: HandType = combinations.iter().fold(parse_hand_type(&*combinations[0]), |current_best, hand| {
            let possible_hand_type: HandType = parse_hand_type(hand);
            if possible_hand_type < current_best { possible_hand_type } else { current_best }
        });
        // println!("Best hand type for {}: {:?}", parts[0], best_hand_type);
        Hand {
            hand_type: best_hand_type,
            value: parts[0],
            bid: parts[1].parse::<u16>().expect("Invalid bid number")
        }
    }).collect();
    let value_mapper_2 : HashMap<char, u8> = HashMap::from(
        [('A', 14),('K', 13),('Q', 12),('T', 10),('9', 9),('8', 8),('7', 7),('6', 6),('5', 5),('4', 4),('3', 3),('2', 2),('J', 1),]
    );
    calculate_total_winnings(&mut parsed_hands_2, &value_mapper_2); // 248029057
}

fn calculate_total_winnings(hands: &mut Vec<Hand>, value_mapper: &HashMap<char, u8>) {
    hands.sort_by(|hand1, hand2| {
        if hand1.hand_type != hand2.hand_type {
            hand2.hand_type.cmp(&hand1.hand_type)
        } else {
            let hand1_values = hand1.value.chars().map(|e| value_mapper[&e]);
            let hand2_values = hand2.value.chars().map(|e| value_mapper[&e]);
            hand1_values.cmp(hand2_values)
        }
    });
    // println!("Sorted hands: {:?}", hands);
    let bids : Vec<_> = hands.iter().enumerate().map(|(index, hand)| {
        (index+1) * hand.bid as usize
    }).collect();
    // println!("Bids: {:?}", bids);
    println!("Total winnings: {}", bids.iter().sum::<usize>())
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    Five, Four, FullHouse, Three, TwoPair, OnePair, High,
}

#[derive(Debug, Clone, PartialEq)]
struct Hand<'a> {
    hand_type: HandType,
    value: &'a str,
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

fn parse_hand_type(hand: &str) -> HandType {
    let mut char_count: HashMap<char, u8> = HashMap::new();
    for c in hand.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    }
    let mut sorted_char_count: Vec<_> = char_count.into_iter().collect();
    sorted_char_count.sort_by(|(_, cnt1), (_, cnt2)| { cnt2.cmp(cnt1) });
    // println!("Sorted char count for {}: {:?}", line, sorted_char_count);
    let (_, top_cnt) = sorted_char_count.first().expect("Unable to get first");
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
    use crate::day07::{HandType, parse_hand_type};

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