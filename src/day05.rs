use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::sync::mpsc;
use std::thread;

pub(crate) fn day5() {
    println!("Day 5");
    let file_path = "resources/day05.txt";
    // let file_path = "resources/test-input.txt";
    let mut file = File::open(file_path).expect("Unable to open file");

    let mut buffer = String::new();
    file.read_to_string( &mut buffer).expect("Unable to read file into string");
    // println!("File contents:\n{}", buffer);

    let lines: Vec<String> = buffer.lines().map(String::from).collect();
    let seeds: Vec<i64> = lines[0].replace("seeds: ", "").split_whitespace().map(|e| {
        e.trim().parse::<i64>().expect("invalid seed")
    }).collect();
    // println!("Seeds: {:?}", seeds);

    let ranges_to_offset = parse_maps(&lines[2..]);
    println!("Ranges to offset mapping: {:?}", ranges_to_offset);

    part_1(&seeds, &ranges_to_offset);
    part_2(&seeds, &ranges_to_offset);
}

fn parse_maps(lines: &[String]) -> Vec<Vec<(Range<i64>, i64)>> {
    let mut numbers: Vec<(i64, i64, i64)> = vec![];
    let mut range_to_offset: Vec<Vec<(Range<i64>, i64)>> = vec![];
    fn build_range_offset(numbers: &Vec<(i64, i64, i64)>) -> Vec<(Range<i64>, i64)> {
        numbers.iter().map(|(destination_range_start, source_range_start, length)| {
            (*source_range_start..(source_range_start+length), (destination_range_start-source_range_start))
        }).collect()
    }
    for line in lines {
        if line.trim().is_empty() {
            range_to_offset.push(build_range_offset(&numbers));
            numbers = vec![];
        } else if line.contains("map") {
            // do nothing
        } else {
            let result: Vec<_> = line.trim().split_whitespace().map(|e| {
                e.trim().parse::<i64>().expect("unable to parse number")
            }).collect();
            numbers.push((result[0], result[1], result[2]));
        }
    }
    range_to_offset.push(build_range_offset(&numbers));
    range_to_offset
}

// [3] Lowest location 146071405
// [9] Received: 146071405
// Lowest location for all seed edges: 104070862
fn part_2(seeds: &Vec<i64>, ranges_to_offset: &Vec<Vec<(Range<i64>, i64)>>) {
    let seed_edges: Vec<(i64,i64)> = seeds.chunks(2).map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1)).collect();
    println!("Seed edges for part 2 {:?}", seed_edges);

    let (sender, receiver) = mpsc::channel();
    let mut index = 0;
    for (low, high) in seed_edges.clone() {
        let sender_clone = sender.clone();
        let range_to_offset_clone = ranges_to_offset.clone();
        thread::spawn(move || {
            let mut min_length_location: i64 = i64::MAX;
            let mut min_seed: i64 = 0;
            println!("[{}] Starting new seed edge low={} high={}", index, low, high);
            for i in low..=high {
                let location = find_in_mappings(&i, &range_to_offset_clone);
                if location < min_length_location {
                    min_length_location = location;
                    min_seed = i;
                }
                if i % 10_000_000 == 0 {
                    println!("[{}] Still working hard...[min seed {} with location {}, current seed {}]", index, min_seed, min_length_location, i)
                }
            }
            println!("[{}] Lowest location {}, seed {}", index, min_length_location, min_seed);
            sender_clone.send(min_length_location).unwrap();
        });
        index += 1;
    }

    let mut result : Vec<i64> = vec![];
    for ind in 0..seed_edges.len() {
        let received_value = receiver.recv().unwrap();
        println!("[{}] Received: {}", ind, received_value);
        result.push(received_value);
    }
    println!("Lowest location for all seed edges: {:?}", result.iter().min().expect("Unable to find min"))
}

fn part_1(seeds: &Vec<i64>, ranges_to_offset: &Vec<Vec<(Range<i64>, i64)>>) {
    let locations: Vec<i64> = seeds.iter().map(|seed| {
        let location: i64 = find_in_mappings(seed, ranges_to_offset);
        // println!("Found target location {}", location);
        location
    }).collect();
    println!("Corresponding locations: {:?}", locations);
    let lowest = locations.iter().min().expect("Unable to find min location");
    println!("Lowest location: {}", lowest);
}

fn find_in_mappings(seed: &i64, ranges_to_offset: &Vec<Vec<(Range<i64>, i64)>>) -> i64 {
    ranges_to_offset.iter().fold(*seed, |temp_location, range_to_offset| {
        let new_location = find_in_mapping(&temp_location, range_to_offset);
        new_location
    })
}

fn find_in_mapping(element: &i64, range_to_offset: &Vec<(Range<i64>, i64)>) -> i64 {
    let target_range = range_to_offset.iter().find(|(range, _)| {
        range.contains(element)
    });
    let result = match target_range {
        Some((_, offset)) => {
            element + offset
        }
        None => { *element }
    };
    result
}

#[cfg(test)]
mod tests {
    use crate::day05::find_in_mapping;

    #[test]
    fn check_find_in_mapping() {
        let result = find_in_mapping(&79, &vec![((50..98), 2)]);
        assert_eq!(result, 81);

        let result2 = find_in_mapping(&120, & vec![((50..98), 2)]);
        assert_eq!(result2, 120);
    }
}
