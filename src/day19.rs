use std::collections::HashMap;

pub(crate) fn day19() {
    println!("Day 19");
    // let file_content = include_str!("../resources/test-input.txt");
    let file_content = include_str!("../resources/day19.txt").trim();
    // println!("File content:\n{}", file_content);

    let workflows: HashMap<String, String> = file_content.lines().take_while(|l| !l.is_empty())
        .map(|w| parse_workflow(w)).collect();
    let parts: Vec<_> = file_content.lines().skip_while(|l| !l.is_empty()).filter(|e| !e.is_empty())
        .map(|p| parse_part(p)).collect();
    // println!("Workflows: {:#?}", workflows);
    // println!("Parts: {:?}", parts);

    // naive_approach(&parts, &workflows);
    let graph = build_node(String::from("in"), &[4000, 4000, 4000, 4000], &[0, 0, 0, 0], &workflows);
    // println!("Graph: {:#?}", graph);

    let sum_parts = parts.iter().map(|part| sum_a_parts(&graph, &part)).sum::<usize>();
    println!("Sum of parts: {:?}", sum_parts);

    let a_edges = recursive_find_a_edges(&graph);
    // println!("A-edges: {:#?}", a_edges);
    let result = a_edges.iter().map(|&edge| {
        edge.max.iter().enumerate().fold(1i64, |acc, (ind, el)| acc * (el - edge.min[ind]) as i64)
    }).collect::<Vec<_>>();
    println!("Result: {:?}", result.iter().sum::<i64>());
}

fn sum_a_parts(node: &Node, part: &[usize; 4]) -> usize {
    // println!("Working with node: {:?} for {:?}", node.name, part);
    if node.name == "A" {
        return part.iter().sum()
    }
    if node.name == "R" {
        return 0
    }
    if node.edges.is_empty() {
        return 0
    }
    let possible = node.edges.iter().filter(|e| {
        e.min.iter().enumerate().all(|(i, val)| val < &part[i])
        &&
        e.max.iter().enumerate().all(|(i, val)| val >= &part[i])
    }).collect::<Vec<_>>();
    if possible.len() == 0 {
        return 0
    }
    if possible.len() != 1 {
        panic!("{}", format!("invalid number of possible edges {:?}", possible))
    }
    sum_a_parts(&possible[0].node, part)
}

fn recursive_find_a_edges(graph: &Node) -> Vec<&Edge> {
    graph.edges.iter().flat_map(|edge| {
        if edge.node.name == "A" {
            vec![edge]
        } else if edge.node.name == "R" {
            vec![]
        } else {
            recursive_find_a_edges(&edge.node)
        }
    }).collect::<Vec<_>>()
}

fn build_node(name: String, max: &[usize; 4], min: &[usize; 4], workflows: &HashMap<String, String>) -> Node {
    if name == "A" || name == "R" {
        return Node { name: String::from(name), edges: vec![] }
    }
    let workflow = workflows.get(&name).unwrap();
    let edges: Vec<Edge> = parse_edges(max, min, workflow, workflows);
    let node = Node { name, edges };
    node
}

fn parse_edges(max: &[usize; 4], min: &[usize; 4], workflow: &String, workflows: &HashMap<String, String>) -> Vec<Edge> {
    let mut result: Vec<Edge> = vec![];
    let edges: Vec<_> = workflow.split(",").collect();
    let mut local_max = *max;
    let mut local_min = *min;
    for edge in edges {
        if !edge.contains(":") {
            let new_edge = Edge {
                min: local_min.clone(),
                max: local_max.clone(),
                node: build_node(String::from(edge), &local_max, &local_min, workflows)
            };
            result.push(new_edge);
        } else {
            let edge_parts: Vec<_> = edge.split(":").collect();
            let chars = edge_parts[0].chars().collect::<Vec<_>>();
            let ch = *chars.get(0).unwrap();
            let index = "xmas".find(*&ch).unwrap();
            let val = edge_parts[0][2..].parse::<usize>().unwrap();
            let mut edge_local_min = local_min.clone();
            let mut edge_local_max = local_max.clone();
            if chars[1] == '>' {
                edge_local_min[index] = val.max(local_min[index]);
                local_max[index] = val.min(local_max[index]);
            } else if chars[1] == '<' {
                edge_local_max[index] = (val-1).min(local_max[index]);
                local_min[index] = (val-1).max(local_min[index]);
            } else {
                panic!("🤯")
            }
            let new_edge = Edge {
                min: edge_local_min.clone(),
                max: edge_local_max.clone(),
                node: build_node(String::from(edge_parts[1]), &edge_local_max, &edge_local_min, workflows)
            };
            result.push(new_edge);
        }
    }
    result
}


#[derive(Debug)]
struct Node {
    name: String,
    edges: Vec<Edge>,
}

#[derive(Debug)]
struct Edge {
    node: Node,
    max: [usize; 4],
    min: [usize; 4],
}

fn parse_workflow(line: &str) -> (String, String) {
    let key: String = line.chars().take_while(|&ch| ch != '{').collect();
    let val: String = line.chars().skip_while(|&ch| ch != '{').collect();
    (key, val.replace('{', "").replace('}', ""))
}

fn parse_part(line: &str) -> [usize;4] {
    let p = line.replace('{', "").replace('}', "").split(",")
        .map(|p1| p1.split("=").collect::<Vec<_>>()[1].parse::<usize>().unwrap()).collect::<Vec<_>>();
    return [p[0], p[1], p[2], p[3]]
}