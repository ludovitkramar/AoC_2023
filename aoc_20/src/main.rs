use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    ops::AddAssign,
};

struct OperationInfo {
    sender: String,
    target: String,
    signal: Signal,
}

fn main() {
    let input = include_str!("input");
    
    let output = part_one(input);
    println!("Part one: {}", output);
}

#[test]
fn test() {
    let example_a = include_str!("example_a");    
    let output = part_one(example_a);
    assert_eq!(output, 32000000);

    let example_b = include_str!("example_b");    
    let output = part_one(example_b);
    assert_eq!(output, 11687500);
}

fn part_one(input: &str) -> u32 {
    let mut components = parse_components(input);

    let mut total_stats = SignalStats {
        low_count: 0,
        high_count: 0,
    };

    for _ in 0..1000 {
        let stats = press_button(&mut components);
        total_stats += stats;
    }

    println!("Stats: {:?}", total_stats);
    total_stats.high_count * total_stats.low_count
}

#[derive(Debug)]
struct SignalStats {
    low_count: u32,
    high_count: u32,
}

impl AddAssign for SignalStats {
    fn add_assign(&mut self, rhs: Self) {
        self.low_count += rhs.low_count;
        self.high_count += rhs.high_count;
    }
}

fn press_button(components: &mut HashMap<String, Box<dyn Component>>) -> SignalStats {
    println!("Components: {:?}", components);

    let mut instructions_queue: VecDeque<OperationInfo> = VecDeque::new();
    instructions_queue.push_back(OperationInfo {
        sender: "button".to_string(),
        target: "broadcaster".to_owned(),
        signal: Signal::Low,
    });

    let mut low_count = 0;
    let mut high_count = 0;

    while !instructions_queue.is_empty() {
        let instruction = instructions_queue.pop_front().unwrap();

        match instruction.signal {
            Signal::High => high_count += 1,
            Signal::Low => low_count += 1,
        }

        let component = components.get_mut(&instruction.target);
        match component {
            Some(component) => {
                let next = component.on_input(&instruction.sender, instruction.signal);
                match next {
                    Some(next) => {
                        for name in next.0 {
                            println!("Adding to queue: {}, {:?}", name, next.1);
                            instructions_queue.push_back(OperationInfo {
                                sender: component.get_name(),
                                target: name,
                                signal: next.1,
                            });
                        }
                    }
                    None => {
                        println!("No output from: {}", instruction.target);
                    }
                }
            }
            None => {
                println!(
                    "No component named: {}. Signal: {:?}",
                    instruction.target, instruction.signal
                );
            }
        }
    }

    SignalStats {
        low_count,
        high_count,
    }
}

fn parse_components(input: &str) -> HashMap<String, Box<dyn Component>> {
    let mut operations: HashMap<String, Box<dyn Component>> = HashMap::new();
    let mut parents: HashMap<String, Vec<String>> = HashMap::new();
    let mut children = HashMap::new();

    for line in input.lines() {
        match line.split("->").map(|s| s.trim()).collect::<Vec<_>>()[..] {
            [opr, targets] => {
                let targets = targets.split(",").map(|t| t.trim()).collect::<Vec<_>>();

                match opr.chars().collect::<Vec<_>>()[..] {
                    ['&', c1, c2] => {
                        let mut name = String::new();
                        name.push(c1);
                        name.push(c2);

                        let conj = Conjunction {
                            name: name.clone(),
                            children: Vec::new(),
                            parent: Vec::new(),
                            memory: HashMap::new(),
                        };

                        for target in targets.iter() {
                            let target = target.to_string();
                            match parents.get_mut(&target) {
                                Some(list) => {
                                    list.push(name.clone());
                                }
                                None => {
                                    parents.insert(target, vec![name.clone()]);
                                }
                            }
                        }
                        children.insert(name.clone(), targets);
                        operations.insert(name, Box::new(conj));
                    }
                    ['%', c1, c2] => {
                        let mut name = String::new();
                        name.push(c1);
                        name.push(c2);

                        // Flip-flop modules (prefix %) are either on or off; they are initially off.
                        let ff = FlipFlop {
                            name: name.clone(),
                            children: Vec::new(),
                            on: false,
                        };

                        for target in targets.iter() {
                            let target = target.to_string();
                            match parents.get_mut(&target) {
                                Some(list) => {
                                    list.push(name.clone());
                                }
                                None => {
                                    parents.insert(target, vec![name.clone()]);
                                }
                            }
                        }
                        children.insert(name.clone(), targets);
                        operations.insert(name, Box::new(ff));
                    }
                    ['b', 'r', 'o', 'a', 'd', 'c', 'a', 's', 't', 'e', 'r'] => {
                        let b = Broadcaster {
                            children: Vec::new(),
                        };

                        children.insert("broadcaster".to_string(), targets);
                        operations.insert("broadcaster".to_string(), Box::new(b));
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    // patch children and parents of operations.
    let mut temp_data = HashMap::new();
    for key in operations.keys() {
        let children = children
            .get(key)
            .unwrap()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let parents = parents.get(key);
        let parents = match parents {
            Some(parents) => parents.clone(),
            None => Vec::new(),
        };

        temp_data.insert(key.clone(), (parents, children));
    }

    for key in temp_data.keys() {
        let operation = operations.get_mut(key).unwrap();
        let data = temp_data.get(key).unwrap();
        let parents = &data.0;
        let children = &data.1;

        for parent in parents {
            operation.add_parent(parent);
        }
        for child in children {
            operation.add_child(child);
        }
    }

    operations
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Signal {
    High,
    Low,
}

#[derive(Debug)]
enum ComponentType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

trait Component: Debug {
    fn get_name(&self) -> String;
    fn get_type(&self) -> ComponentType;
    fn add_child(&mut self, child: &String);
    fn add_parent(&mut self, parent: &String);
    fn on_input(&mut self, sender: &String, signal: Signal) -> Option<(Vec<String>, Signal)>;
}

#[derive(Debug)]
/// If a flip-flop module receives a high pulse, it is ignored and nothing happens. However, if a flip-flop module receives a low pulse, it flips between on and off. If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.
struct FlipFlop {
    name: String,
    on: bool,
    children: Vec<String>,
}

impl Component for FlipFlop {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> ComponentType {
        ComponentType::FlipFlop
    }

    fn on_input(&mut self, _: &String, signal: Signal) -> Option<(Vec<String>, Signal)> {
        match signal {
            Signal::High => None,
            Signal::Low => {
                match self.on {
                    true => {
                        // turns off and sends a low pulse.
                        self.on = false;
                        Some((self.children.clone(), Signal::Low))
                    }
                    false => {
                        // turns on and sends a high pulse.
                        self.on = true;
                        Some((self.children.clone(), Signal::High))
                    }
                }
            }
        }
    }

    fn add_child(&mut self, child: &String) {
        self.children.push(child.clone());
    }

    fn add_parent(&mut self, _: &String) {}
}

/// Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their connected input modules; they initially default to remembering a low pulse for each input. When a pulse is received, the conjunction module first updates its memory for that input. Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
#[derive(Debug)]
struct Conjunction {
    name: String,
    children: Vec<String>,
    parent: Vec<String>,
    memory: HashMap<String, Signal>,
}

impl Component for Conjunction {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> ComponentType {
        ComponentType::Conjunction
    }

    fn add_child(&mut self, child: &String) {
        self.children.push(child.clone());
    }

    fn add_parent(&mut self, parent: &String) {
        self.parent.push(parent.clone());
        // They initially default to remembering a low pulse for each input.
        self.memory.insert(parent.clone(), Signal::Low);
    }

    fn on_input(&mut self, sender: &String, signal: Signal) -> Option<(Vec<String>, Signal)> {
        match self.memory.get(sender) {
            Some(_) => {
                // When a pulse is received, the conjunction module first updates its memory for that input.
                self.memory.insert(sender.to_owned(), signal);
                // Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
                let pulse = if self.memory.values().any(|&s| s == Signal::Low) {
                    Signal::High
                } else {
                    Signal::Low
                };
                Some((self.children.clone(), pulse))
            }
            None => panic!(
                "Expected to have some memory, was add_parent called? \n Component: {}. Sender: {}. Memory: {:?}",
                self.name, sender, self.memory
            ),
        }
    }
}

#[derive(Debug)]
struct Broadcaster {
    children: Vec<String>,
}
impl Component for Broadcaster {
    fn get_name(&self) -> String {
        "broadcaster".to_string()
    }

    fn get_type(&self) -> ComponentType {
        ComponentType::Broadcaster
    }

    fn add_child(&mut self, child: &String) {
        self.children.push(child.clone());
    }

    fn add_parent(&mut self, _: &String) {}

    fn on_input(&mut self, _: &String, signal: Signal) -> Option<(Vec<String>, Signal)> {
        Some((self.children.clone(), signal))
    }
}
