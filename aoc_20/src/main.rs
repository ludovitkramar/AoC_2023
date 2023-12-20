use std::{collections::HashMap, fmt::Debug};

fn main() {
    println!("Hello, world!");

    let input = include_str!("example_a");

    let mut operations: HashMap<String, (Box<dyn Component>, Vec<&str>)> = HashMap::new();

    for line in input.lines() {
        match line.split("->").map(|s| s.trim()).collect::<Vec<_>>()[..] {
            [opr, targets] => {
                let targets = targets.split(",").collect::<Vec<_>>();                

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

                        operations.insert(name, (Box::new(conj), targets));
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
                        operations.insert(name, (Box::new(ff), targets));
                    }
                    ['b', 'r', 'o', 'a', 'd', 'c', 'a', 's', 't', 'e', 'r'] => {
                        println!("BROADCASTER: {:?}", targets);

                        let b = Broadcaster {
                            children: Vec::new(),
                        };

                        operations.insert("broadcaster".to_string(), (Box::new(b), targets));
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    println!("Operations: {:?}", operations);

    // TODO: patch children and parents of operations.
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
    fn add_child(&mut self, child: &'static mut dyn Component);
    fn add_parent(&mut self, parent: &'static dyn Component);
    fn on_input(&mut self, sender: &String, signal: Signal);
}

#[derive(Debug)]
/// If a flip-flop module receives a high pulse, it is ignored and nothing happens. However, if a flip-flop module receives a low pulse, it flips between on and off. If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.
struct FlipFlop<'a> {
    name: String,
    on: bool,
    children: Vec<Box<&'a mut dyn Component>>,
}

impl Component for FlipFlop<'_> {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> ComponentType {
        ComponentType::FlipFlop
    }

    fn on_input(&mut self, _: &String, signal: Signal) {
        match signal {
            Signal::High => {}
            Signal::Low => {
                match self.on {
                    true => {
                        // turns off and sends a low pulse.
                        self.on = false;
                        for child in self.children.iter_mut() {
                            child.on_input(&self.name, Signal::Low);
                        }
                    }
                    false => {
                        // turns on and sends a high pulse.
                        self.on = true;
                        for child in self.children.iter_mut() {
                            child.on_input(&self.name, Signal::High);
                        }
                    }
                }
            }
        }
    }

    fn add_child(&mut self, child: &'static mut dyn Component) {
        self.children.push(Box::new(child));
    }

    fn add_parent(&mut self, _: &'static dyn Component) {}
}

/// Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their connected input modules; they initially default to remembering a low pulse for each input. When a pulse is received, the conjunction module first updates its memory for that input. Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
#[derive(Debug)]
struct Conjunction<'a> {
    name: String,
    children: Vec<Box<&'a mut dyn Component>>,
    parent: Vec<Box<&'a dyn Component>>,
    memory: HashMap<String, Signal>,
}

impl Component for Conjunction<'_> {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> ComponentType {
        ComponentType::Conjunction
    }

    fn add_child(&mut self, child: &'static mut dyn Component) {
        self.children.push(Box::new(child));
    }

    fn add_parent(&mut self, parent: &'static dyn Component) {
        self.parent.push(Box::new(parent));
        // They initially default to remembering a low pulse for each input.
        self.memory
            .insert(parent.get_name().to_owned(), Signal::Low);
    }

    fn on_input(&mut self, sender: &String, signal: Signal) {
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
                for child in self.children.iter_mut() {
                    child.on_input(&self.name, pulse);
                }
            }
            None => panic!("Expected to have some memory, was add_parent called?"),
        }
    }
}

#[derive(Debug)]
struct Broadcaster<'a> {
    children: Vec<Box<&'a mut dyn Component>>,
}
impl Component for Broadcaster<'_> {
    fn get_name(&self) -> String {
        "broadcaster".to_string()
    }

    fn get_type(&self) -> ComponentType {
        ComponentType::Broadcaster
    }

    fn add_child(&mut self, child: &'static mut dyn Component) {
        self.children.push(Box::new(child));
    }

    fn add_parent(&mut self, _: &'static dyn Component) {}

    fn on_input(&mut self, _: &String, signal: Signal) {
        let name = self.get_name();
        for child in self.children.iter_mut() {
            child.on_input(&name, signal.clone());
        }
    }
}

#[derive(Debug)]
struct Button<'a> {
    broadcaster: &'a mut Broadcaster<'a>,
}

impl Component for Button<'_> {
    fn get_name(&self) -> String {
        "button".to_string()
    }

    fn get_type(&self) -> ComponentType {
        ComponentType::Button
    }

    fn add_child(&mut self, _: &'static mut dyn Component) {}
    fn add_parent(&mut self, _: &'static dyn Component) {}
    fn on_input(&mut self, _: &String, _: Signal) {}
}

impl Button<'_> {
    fn press(&mut self) {
        self.broadcaster.on_input(&self.get_name(), Signal::Low);
    }
}
