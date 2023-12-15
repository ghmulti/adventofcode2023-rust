pub(crate) fn day15() {
    println!("Day 15");
    // let file_content = include_str!("../resources/test-input.txt");
    let file_content = include_str!("../resources/day15.txt").trim();
    // println!("File content:\n[START]{}[END]", file_content);

    let steps: Vec<_> = file_content.split(',').map(|e| { hash_str(e) }).collect();
    println!("Sum of steps: {}", steps.iter().sum::<usize>());

    let mut boxes:Vec<Vec<(&str, usize)>> = (0..256).map(|_| vec![]).collect();
    for e in file_content.split(',') {
        if e.contains('=') {
            let parts = e.split('=').collect::<Vec<_>>();
            let val = parts[1].parse::<usize>().unwrap();
            let h = hash_str(parts[0]);
            let contains = boxes[h].iter().any(|(e, _)| { *e == parts[0] });
            if contains {
                boxes[h] = boxes[h].iter().map(|el| {
                    if el.0 == parts[0] {
                        (el.0, val) } else { *el }
                }).collect();
            } else {
                boxes[h].push((parts[0], val))
            }
        } else if e.contains('-') {
            let parts = e.split('-').collect::<Vec<_>>();
            let h = hash_str(parts[0]);
            boxes[h].retain(|(s, _)| *s != parts[0]);
        } else {
            panic!("Invalid input")
        }
    }
    // println!("Boxes: {:?}", boxes);

    let result: usize = boxes.iter().enumerate().map(|(box_index, bx)| {
        bx.iter().enumerate().map(|(index, (_, val))| (box_index+1)*(index+1)*val).sum::<usize>()
    }).sum();
    println!("Focusing power of the resulting lens configuration: {}", result);
}

fn hash_str(str: &str) -> usize {
    str.chars().fold(0, |mut acc, ch| { hash(acc, ch) })
}

fn hash(mut acc: usize, ch: char) -> usize {
    let ascii = ch as u8;
    acc += ascii as usize;
    acc *= 17;
    acc = acc % 256;
    acc
}