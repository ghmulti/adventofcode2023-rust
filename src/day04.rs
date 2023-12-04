use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub(crate) fn day4() {
    println!("Day 4");
    let file_path = "resources/day04.txt";
    // let file_path = "resources/test-input.txt";
    let mut file = File::open(file_path).expect("Unable to open file");

    let mut buffer = String::new();
    file.read_to_string( &mut buffer).expect("Unable to read file into string");
    // println!("File contents:\n{}", buffer);

    let lines: Vec<String> = buffer.lines().map(String::from).collect();
    let game = lines.iter().fold(Game { cards: Vec::new() }, |mut acc, line| {
        let card = parse_line(line);
        acc.cards.push(card);
        acc
    });
    // println!("{:?}", game);

    part_1(&game);
    part_2(&game);
}

#[derive(Debug)]
struct Game {
    cards: Vec<Card>
}

#[derive(Debug)]
struct Card {
    index: u16,
    winning_numbers: Vec<u16>,
    your_numbers: Vec<u16>,
}

fn parse_line(line: &String) -> Card {
    let card_and_numbers: Vec<_> = line.split(":").collect();
    let card_index = card_and_numbers[0].replace("Card ", "").trim().parse::<u16>().expect("Invalid index");
    let all_numbers: Vec<_> = card_and_numbers[1].split("|").collect();
    let winning_numbers: Vec<_> = all_numbers[0].trim().split_whitespace().map(|e| {
        e.trim().parse::<u16>().expect("Invalid number")
    }).collect();
    let your_numbers: Vec<_> = all_numbers[1].trim().split_whitespace().map(|e| {
        e.trim().parse::<u16>().expect("Invalid number")
    }).collect();
    Card {
        index: card_index,
        winning_numbers,
        your_numbers,
    }
}

fn part_1(game: &Game) {
    let number_of_points: u16 = game.cards.iter().map(|card| {
        let your_winning_numbers: Vec<_> = card.your_numbers.iter().filter(|your_number| {
            card.winning_numbers.contains(your_number)
        }).collect();
        // println!("Your winning numbers for {}: {:?}", card.index, your_winning_numbers);
        let points : u16 = your_winning_numbers.iter().fold(0, |acc, _| {
            if acc == 0 { 1 } else { acc * 2 }
        });
        // println!("Number of points: {}", points);
        points
    }).sum();
    println!("Sum of points: {}", number_of_points);
}

fn part_2(game: &Game) {
    let mut index_with_numbers: Vec<_> = game.cards.iter().map(|card| {
        let your_winning_numbers: Vec<_> = card.your_numbers.iter().filter(|your_number| {
            card.winning_numbers.contains(your_number)
        }).collect();
        (card.index, your_winning_numbers.len() as u16)
    }).collect();
    let index_to_score: HashMap<_, _> = index_with_numbers.clone().into_iter().collect();
    // println!("Original cards with points: {:?}", index_with_numbers);
    let mut index = 0;
    while index < index_with_numbers.len() {
        let (current_ind, current_score) = index_with_numbers[index];
        let to_add = ((current_ind+1)..=(current_ind + current_score)).collect::<Vec<_>>();
        // println!("To add: {:?}", to_add);
        for element in to_add {
            let val = index_to_score[&element];
            index_with_numbers.push((element, val));
        }
        index += 1;
    }
    // println!("Cards with points: {:?}", index_with_numbers);
    println!("Total number of scratchcards: {}", index_with_numbers.len())
}


#[cfg(test)]
mod tests {
    use crate::day04::parse_line;

    #[test]
    fn parse_line_check() {
        let line = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let card = parse_line(&line);
        assert_eq!(card.index, 1);
        assert_eq!(card.your_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
        assert_eq!(card.winning_numbers, vec![41, 48, 83, 86, 17]);
    }
}