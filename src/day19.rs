use std::collections::HashMap;

pub(crate) fn day19() {
    println!("Day 19");
    // let file_content = include_str!("../resources/test-input.txt");
    let file_content = include_str!("../resources/day19.txt").trim();
    println!("File content:\n{}", file_content);

    let workflows: HashMap<String, String> = file_content.lines().take_while(|l| !l.is_empty())
        .map(|w| parse_workflow(w)).collect();
    let parts: Vec<_> = file_content.lines().skip_while(|l| !l.is_empty()).filter(|e| !e.is_empty())
        .map(|p| parse_part(p)).collect();
    println!("Workflows: {:?}", workflows);
    println!("Parts: {:?}", parts);

    let result: Vec<_> = parts.iter().map(|part| {
        let end = process_workflow("in", part, &workflows);
        if end == "A" {
            part.0 + part.1 + part.2 + part.3
        } else {
            0
        }
    }).collect();
    println!("Results: {:?}", result.iter().sum::<usize>());
}

fn process_workflow<'a>(key: &'a str, part: &(usize, usize, usize, usize), workflows: &'a HashMap<String, String>) -> &'a str {
    if key == "R"  || key == "A" {
        return key
    }
    let rule = workflows.get(key).expect("Unable to find key in map");
    for rule_part in rule.split(",").collect::<Vec<_>>() {
        if !rule_part.contains(":") {
            return process_workflow(rule_part, part, workflows)
        } else {
            let pp = rule_part.split(":").collect::<Vec<_>>();
            if evaluate(part, pp[0]) {
                return process_workflow(pp[1], part, workflows)
            }
        }
    }
    panic!("🤯")
}

fn evaluate((x, m, a, s): &(usize, usize, usize, usize), p: &str) -> bool {
    let v = &p[2..].parse::<usize>().unwrap();
    let ch = p.chars().collect::<Vec<_>>();
    match ch[0] {
        'x' => {
            if ch[1] == '>' { x > v } else { x < v }
        }
        'm' => {
            if ch[1] == '>' { m > v } else { m < v }
        }
        'a' => {
            if ch[1] == '>' { a > v } else { a < v }
        }
        's' => {
            if ch[1] == '>' { s > v } else { s < v }
        }
        _ => panic!("🤯")
    }
}

fn parse_workflow(line: &str) -> (String, String) {
    let key: String = line.chars().take_while(|&ch| ch != '{').collect();
    let val: String = line.chars().skip_while(|&ch| ch != '{').collect();
    (key, val.replace('{', "").replace('}', ""))
}

fn parse_part(line: &str) -> (usize, usize, usize, usize) {
    let p = line.replace('{', "").replace('}', "").split(",")
        .map(|p1| p1.split("=").collect::<Vec<_>>()[1].parse::<usize>().unwrap()).collect::<Vec<_>>();
    return (p[0], p[1], p[2], p[3])
}