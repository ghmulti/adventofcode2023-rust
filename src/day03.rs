use std::fs::File;
use std::io::Read;

pub(crate) fn day3() {
    println!("Day 3");
    let file_path = "resources/day03.txt";
    // let file_path = "resources/test-input.txt";
    let mut file = File::open(file_path).expect("Unable to open file");

    let mut buffer = String::new();
    file.read_to_string( &mut buffer).expect("Unable to read file into string");
    // println!("File contents:\n{}", buffer);

    let lines: Vec<String> = buffer.lines().map(String::from).collect();
    let engine = lines.iter().enumerate().fold(Engine {
        part_numbers: Vec::new(),
        symbols: Vec::new(),
    }, |mut eng, (index, line)| {
        let (parts, symbols) = parse_line(index, line);
        eng.symbols.extend(symbols);
        eng.part_numbers.extend(parts);
        eng
    });

    // println!("{:?}", engine);
    part_1(&engine);
    part_2(&engine);
}

#[derive(Debug)]
struct Engine {
    part_numbers: Vec<PartNumberMaybe>,
    symbols: Vec<Symbol>,
}

#[derive(Debug)]
struct PartNumberMaybe {
    value: i32,
    indexes: Vec<(i16, i16)>,
}

#[derive(Debug)]
struct Symbol {
    value: char,
    index: (i16, i16),
}

fn parse_line(row_index: usize, line: &String) -> (Vec<PartNumberMaybe>, Vec<Symbol>) {
    // TODO: refactor duplicate code
    // TODO: maybe use generators / channels?
    let mut numbers: Vec<PartNumberMaybe> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut current_number_indexes: Vec<(i16, i16)> = Vec::new();
    let mut current_number = String::new();
    for (col_index, ch) in format!("{}..", line).chars().enumerate() {
        if ch == '.' {
            if current_number.len() > 0 {
                let value = current_number.parse::<i32>().expect("Invalid number");
                numbers.push(PartNumberMaybe {
                    value,
                    indexes: current_number_indexes.clone()
                });
                current_number.clear();
                current_number_indexes.clear();
            }
        } else if ch.is_digit(10) {
            current_number.push(ch);
            current_number_indexes.push((row_index as i16, col_index as i16));
        } else {
            if current_number.len() > 0 {
                let value = current_number.parse::<i32>().expect("Invalid number");
                numbers.push(PartNumberMaybe {
                    value,
                    indexes: current_number_indexes.clone()
                });
                current_number.clear();
                current_number_indexes.clear();
            }
            symbols.push(Symbol{value: ch, index: (row_index as i16, col_index as i16)})
        }
    }
    (numbers, symbols)
}

fn part_1(engine: &Engine) {
    let actual_part_numbers = engine.part_numbers.iter().filter(|part_number| {
        engine.symbols.iter().any(|symbol| {
            is_adjacent(part_number, symbol)
        })
    });
    let values: Vec<i32> = actual_part_numbers.map(|part| { part.value }).collect();
    // println!("Filtered part numbers: {:?}", values);
    let result: i32 = values.into_iter().sum();
    println!("Sum of part numbers: {}", result)
}

fn part_2(engine: &Engine) {
    let gear_ratios: i32 = engine.symbols.iter()
        .filter(|s| { s.value == '*' })
        .map(|s| {
            let adjacent_part_numbers = engine.part_numbers.iter().filter(|pn| {
                is_adjacent(pn, s)
            }).collect::<Vec<_>>();
            if adjacent_part_numbers.len() == 2 {
                adjacent_part_numbers[0].value * adjacent_part_numbers[1].value
            } else {
                0
            }
        }).sum();
    println!("Sum of all gear ratios: {}", gear_ratios);
}

fn is_adjacent(part_number_maybe: &PartNumberMaybe, symbol: &Symbol) -> bool {
    let (row, column) = symbol.index;
    let adjacent_positions = [
        (row-1, column-1),
        (row-1, column),
        (row-1, column+1),
        (row, column-1),
        (row, column+1),
        (row+1, column-1),
        (row+1, column),
        (row+1, column+1),

    ];
    part_number_maybe.indexes.iter().any(|ind| {
        adjacent_positions.iter().any(|adj| { adj == ind })
    })
}