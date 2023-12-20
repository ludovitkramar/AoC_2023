use std::{collections::HashMap, fmt::Debug, rc::{Rc, Weak}};

fn main() {
    println!("Hello, world!");

    let input = include_str!("example_a");

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
                        println!("Conjunction Op: [{}]", name);

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
                                },
                                None => {
                                    parents.insert(target, vec![name.clone()]);
                                },
                            }
                        }   
                        children.insert(name.clone(), targets);
                        operations.insert(name, Box::new(conj));
                    }
                    ['%', c1, c2] => {
                        let mut name = String::new();
                        name.push(c1);
                        name.push(c2);
                        println!("Flip-flop Op: [{}]", name);

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
                                },
                                None => {
                                    parents.insert(target, vec![name.clone()]);
                                },
                            }
                        }   
                        children.insert(name.clone(), targets);
                        operations.insert(name, Box::new(ff));
                    }
                    ['b', 'r', 'o', 'a', 'd', 'c', 'a', 's', 't', 'e', 'r'] => {
                        println!("BROADCASTER: {:?}", targets);

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

    println!("Operations: {:?}", operations);
    println!("Parents: {:?}", parents);

    // patch children and parents of operations.
    let mut temp_data = HashMap::new();
    for key in operations.keys() {
        let operation = operations.get(key).unwrap();
        let child = children.get(key).unwrap().iter().map(|s| s.to_string()).collect::<Vec<String>>();
        //let mut children = Vec::new();
        // for target_name in operation.1.iter() {
        //     let target = operations.get(&target_name.to_string());
        //     let child = match target {
        //         Some(target) => {
        //             &target.0
        //         },
        //         None => {
        //             panic!("Couldn't find target: {:?}", target_name);
        //         },
        //     };
        //     children.push(child.get_name().clone());                       
        // }
        let parents = parents.get(key);
        let parents = match parents {
            Some(parents) => parents.clone(),
            None => Vec::new()
        };

        println!("Key: {} has {:?} as parents and {:?} as children.", key, parents, children);
        temp_data.insert(key.clone(), (parents, child));
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
    Button,
}

trait Component: Debug {
    fn get_name(&self) -> String;
    fn get_type(&self) -> ComponentType;
    fn add_child(&mut self, child: &String);
    fn add_parent(&mut self, parent: &String);
    fn on_input(&mut self, sender: &String, signal: Signal, data: &mut HashMap<String, Box<dyn Component>>);    
}

#[derive(Debug)]
/// If a flip-flop module receives a high pulse, it is ignored and nothing happens. However, if a flip-flop module receives a low pulse, it flips between on and off. If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.
struct FlipFlop {
    name: String,
    on: bool,
    children: Vec<String>,
    
}

impl Component for FlipFlop{
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> ComponentType {
        ComponentType::FlipFlop
    }

    fn on_input(&mut self, _: &String, signal: Signal, data: &mut HashMap<String, Box<dyn Component>>) {
        match signal {
            Signal::High => {}
            Signal::Low => {
                match self.on {
                    true => {
                        // turns off and sends a low pulse.
                        self.on = false;
                        for child in self.children.iter() {
                            match data.get_mut(child) {
                                Some(child) => {
                                    child.on_input(&self.name, Signal::Low, data)
                                },
                                None => {
                                    println!("CHILD NOT FOUND {}: {:?}", child, signal);
                                },
                            }                             
                        }
                    }
                    false => {
                        // turns on and sends a high pulse.
                        self.on = true;
                        for child in self.children.iter() {
                            match data.get(child) {
                                Some(child) => {
                                    child.on_input(&self.name, Signal::High, data)
                                },
                                None => {
                                    println!("CHILD NOT FOUND {}: {:?}", child, signal);
                                },
                            }    
                        }
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

impl Component for Conjunction{
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
        self.memory
            .insert(parent.clone(), Signal::Low);
    }

    fn on_input(&mut self, sender: &String, signal: Signal, data: &mut HashMap<String, Box<dyn Component>>) {
        match self.memory.get(sender) {
            Some(_) => {
                self.memory.insert(sender.to_owned(), signal);
                // When a pulse is received, the conjunction module first updates its memory for that input.
                // Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.

                let pulse = if self.memory.values().any(|&s| s == Signal::Low) {
                    Signal::High
                } else {
                    Signal::Low
                };
                for child in self.children.iter() {
                    // child.on_input(&self.name, pulse);
                }
            }
            None => panic!("Expected to have some memory, was add_parent called?"),
        }
    }
    
}

#[derive(Debug)]
struct Broadcaster {
    children: Vec<String>,
    
}
impl Component for Broadcaster{
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

    fn on_input(&mut self, _: &String, signal: Signal, data: &mut HashMap<String, Box<dyn Component>>) {
        let name = self.get_name();
        for child in self.children.iter() {
            // child.on_input(&name, signal.clone());
        }
    }
    
}

#[derive(Debug)]
struct Button {
    children: Vec<Box<dyn Component>>,    
    
}

impl Component for Button {
    fn get_name(&self) -> String {
        "button".to_string()
    }

    fn get_type(&self) -> ComponentType {
        ComponentType::Button
    }

    fn add_child(&mut self, _: &String) {}
    fn add_parent(&mut self, _: &String) {}
    fn on_input(&mut self, _: &String, _: Signal, _: &mut HashMap<String, Box<dyn Component>>) {}
    
}

impl Button {
    fn press(&mut self) {
        let name ="button".to_string();
        for child in self.children.iter() {
            // child.on_input(&name, Signal::Low);
        }
    }
}
