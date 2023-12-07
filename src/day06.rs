pub(crate) fn day6() {
    println!("Day 6");

    // let time_to_distance: Vec<(i64, i64)> = vec![(7,9), (15,40), (30,200)];
    let time_to_distance: Vec<(i64, i64)> = vec![(55,246), (82,1441), (64,1012), (90,1111)];
    part_1(&time_to_distance);

    // let time_to_distance2: (i64, i64) = (71530, 940200);
    let time_to_distance2: (i64, i64) = (55826490, 246144110121111);
    part_1(&vec![time_to_distance2])
}

fn part_1(time_to_distance: &Vec<(i64, i64)>) {
    let number_of_ways: Vec<i64> = time_to_distance.iter().fold(vec![], |mut acc, (race_time, record_distance)| {
        let win_races: usize = build_combinations(race_time).iter()
            .map(|(a, b)| { a * b })
            .filter(|e| { e > record_distance })
            .count();
        // println!("Win races: {:?}", win_races);
        acc.push(win_races as i64);
        acc
    });
    let total_number_of_ways: i64 = number_of_ways.iter().fold(1, |acc, e| { acc * (*e as i64) });
    println!("Number of ways to beat the record: {}", total_number_of_ways)
}

fn build_combinations(race_time: &i64) -> Vec<(i64, i64)> {
    let mut result: Vec<(i64, i64)> = vec![(0, 0)];
    for i in 1..=*race_time {
        result.push((i, race_time-i));
    }
    // println!("Combinations: {:?}", result);
    result
}

#[cfg(test)]
mod tests {
    use crate::day06::build_combinations;

    #[test]
    fn check_combinations() {
        assert_eq!(build_combinations(&7), vec![(0, 0), (1, 6), (2, 5), (3, 4), (4, 3), (5, 2), (6, 1), (7, 0)]);
    }
}
