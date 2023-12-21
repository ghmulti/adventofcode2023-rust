use std::collections::{HashMap, VecDeque};

pub(crate) fn day20() {
    println!("Day 20");
    // let file_content = include_str!("../resources/test-input.txt");
    let file_content = include_str!("../resources/day20.txt").trim();
    // println!("File content:\n{}", file_content);

    let dict: Vec<_> = file_content.lines().map(|line| {
        let m = parse_line(line);
        (m.name(), m.destinations())
    }).collect();
    let mut modules: Vec<_> = file_content.lines().map(|line| {
        parse_line(line)
    }).collect();
    for module in modules.iter_mut() {
        if module.t() == "&" {
            let sources = dict.iter().filter(|(_, dest)| dest.contains(&module.name())).map(|(name, _)| name).collect::<Vec<_>>();
            *module = Box::new(ConjunctionModule {
                name: module.name(),
                memory: HashMap::new(),
                counter: sources.len(),
                destinations: module.destinations()
            })
        }
    }
    // println!("Modules: {:#?}", modules);

    part_1(&mut modules);

    part_2(&mut modules);
}

fn part_2(modules: &mut Vec<Box<dyn SignalRelay>>) {
    let rx_targets = modules.iter().filter_map(|e| {
        if e.destinations().contains(&String::from("rx")) {
            Some(e.name())
        } else {
            None
        }
    }).collect::<Vec<_>>();
    println!("Modules that sends signal to rx: {:?}", rx_targets); // ["jz"]

    let jz_targets = modules.iter().filter_map(|e| {
        if e.destinations().contains(&String::from("jz")) {
            Some(e.name())
        } else {
            None
        }
    }).collect::<Vec<_>>();
    println!("Modules that sends signal to jz: {:?}", jz_targets); // ["dh", "mk", "vf", "rn"]

    // high signals on ["dh", "mk", "vf", "rn"]

    // High signal for vf, number of button pressed 3847
    // High signal for rn, number of button pressed 3923
    // High signal for dh, number of button pressed 4001
    // High signal for mk, number of button pressed 4091
    println!("Fewest button presses to send low signal to rx: {}", [3847usize,3923,4001,4091].iter().fold(1usize, |acc, e| acc * e))
}


fn part_1(modules: &mut Vec<Box<dyn SignalRelay>>) {
    let mut counter_low = 0;
    let mut counter_high = 0;
    for i in 0..5000 {
        let mut relay_to_do: VecDeque<(String, String, u8)> = VecDeque::new();
        relay_to_do.push_front((String::from("button"), String::from("broadcaster"), SIGNAL_LOW));
        counter_low += 1;
        while !relay_to_do.is_empty() {
            // println!("Queue: {:?}", relay_to_do);
            let (source, target, signal) = relay_to_do.pop_front().unwrap();
            // println!("{} --{}--> {}", source, if signal == SIGNAL_LOW { "low" } else { "high"}, target);
            let module_opt = modules.iter_mut().find(|e| e.name() == target.clone());
            if module_opt.is_none() {
                // println!("Unable to find {} in modules, skipping", target);
                continue
            }
            let module = module_opt.unwrap();
            let mut result = module.relay(&source, signal.clone());
            // println!("Result: {:?}", result);
            if result.is_none() {
                // println!("No relay from {}", target);
                continue
            }
            let (new_signal, new_targets) = result.unwrap();
            if new_signal == SIGNAL_LOW {
                counter_low += new_targets.len();
            } else {
                if ["dh", "mk", "vf", "rn"].contains(&&*target) {
                    println!("High signal for {}, number of button pressed {}", target, i+1);
                }
                counter_high += new_targets.len();
            }
            for new_target in new_targets {
                relay_to_do.push_back((target.clone(), new_target.clone(), new_signal));
            }
        }
        // println!("Modules: {:#?}", modules);
    }
    println!("Finished invocation: counter_low {} counter_high {}, multiply {}", counter_low, counter_high, counter_high * counter_low)
}


fn parse_line(line: &str) -> Box<dyn SignalRelay> {
    let parts: Vec<_> = line.split("->").collect::<Vec<_>>();
    let destinations: Vec<_> = parts[1].split(",").map(|e| String::from(e.trim())).collect::<Vec<_>>();
    if parts[0].starts_with("%") {
        let name = String::from(parts[0].replace("%", "").trim());
        Box::new(FlipFlopModule { name, power: false, init: false, destinations })
    } else if parts[0].starts_with("&") {
        let name = String::from(parts[0].replace("&", "").trim());
        Box::new(ConjunctionModule { name, memory: HashMap::new(), counter: 0, destinations })
    } else if parts[0].trim() == "broadcaster" {
        Box::new(BroadcastModule { name: String::from("broadcaster"), destinations })
    } else {
        panic!("🤯")
    }
}

const SIGNAL_LOW: u8 = 0;
const SIGNAL_HIGH: u8 = 1;


trait SignalRelay: std::fmt::Debug {
    fn relay(&mut self, from: &String, signal: u8) -> Option<(u8, Vec<String>)>;
    fn name(&self) -> String;
    fn destinations(&self) -> Vec<String>;
    fn t(&self) -> &str;
}

#[derive(Clone, Debug)]
struct BroadcastModule {
    name: String,
    destinations: Vec<String>,
}

impl SignalRelay for BroadcastModule {
    fn relay(&mut self, from: &String, signal: u8) ->  Option<(u8, Vec<String>)> {
        // println!("Received signal {:?} from {}", signal, from);
        Some((signal, self.destinations.clone()))
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn destinations(&self) -> Vec<String> {
        self.destinations.clone()
    }

    fn t(&self) -> &str {
        "br"
    }
}

#[derive(Clone, Debug)]
struct FlipFlopModule {
    name: String,
    power: bool,
    destinations: Vec<String>,
    init: bool,
}

impl SignalRelay for FlipFlopModule {
    fn relay(&mut self, from: &String, signal: u8) -> Option<(u8, Vec<String>)> {
        // println!("Received signal {:?} from {}", signal, from);
        if signal == SIGNAL_HIGH {
            return None
        }
        self.power = !self.power;
        Some((if self.power { SIGNAL_HIGH } else { SIGNAL_LOW }, self.destinations.clone()))
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn destinations(&self) -> Vec<String> {
        self.destinations.clone()
    }

    fn t(&self) -> &str {
        "%"
    }
}

#[derive(Clone, Debug)]
struct  ConjunctionModule {
    name: String,
    destinations: Vec<String>,
    memory: HashMap<String, u8>,
    counter: usize,
}

impl SignalRelay for ConjunctionModule {
    fn relay(&mut self, from: &String, signal: u8) -> Option<(u8, Vec<String>)> {
        // println!("Received signal {:?} from {}", signal, from);
        self.memory.insert(from.clone(), signal);
        let impuls_to_send = if self.counter == self.memory.len() && self.memory.values().all(|&s| s == SIGNAL_HIGH) { SIGNAL_LOW } else { SIGNAL_HIGH };
        Some((impuls_to_send, self.destinations.clone()))
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn destinations(&self) -> Vec<String> {
        self.destinations.clone()
    }

    fn t(&self) -> &str {
        "&"
    }
}