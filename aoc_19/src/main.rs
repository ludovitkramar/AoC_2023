use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}
impl Data {
    fn get(&self, prop: &Property) -> i32 {
        match prop {
            Property::X => self.x,
            Property::M => self.m,
            Property::A => self.a,
            Property::S => self.s,
        }
    }
}

#[derive(Debug)]
enum Property {
    X,
    M,
    A,
    S,
}

impl FromStr for Property {
    type Err = ParseOprError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Property::A),
            "m" => Ok(Property::M),
            "s" => Ok(Property::S),
            "x" => Ok(Property::X),
            _ => Err(ParseOprError),
        }
    }
}

#[derive(Debug)]
enum Operation {
    LessThan(Property, i32, Box<Operation>),
    MoreThan(Property, i32, Box<Operation>),
    Reject,
    Accept,
    Jump(String),
}

#[derive(Debug)]
struct ParseOprError;
impl FromStr for Operation {
    type Err = ParseOprError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            let data = s.split(':').collect::<Vec<_>>();
            let function_name = data.get(1).ok_or(ParseOprError)?;
            let jump_to = match function_name {
                &"A" => Operation::Accept,
                &"R" => Operation::Reject,
                _ => Operation::Jump(function_name.to_string()),
            };

            let operation = data.get(0).ok_or(ParseOprError)?;

            if operation.contains('>') {
                match operation.split('>').collect::<Vec<_>>()[..] {
                    [prop, value] => {
                        let num = value.parse::<i32>().map_err(|_| ParseOprError)?;
                        let prop = prop.parse::<Property>()?;

                        Ok(Operation::MoreThan(prop, num, Box::new(jump_to)))
                    }
                    _ => Err(ParseOprError),
                }
            } else {
                match operation.split('<').collect::<Vec<_>>()[..] {
                    [prop, value] => {
                        let num = value.parse::<i32>().map_err(|_| ParseOprError)?;
                        let prop = prop.parse::<Property>()?;

                        Ok(Operation::LessThan(prop, num, Box::new(jump_to)))
                    }
                    _ => Err(ParseOprError),
                }
            }
        } else {
            match s {
                "A" => Ok(Operation::Accept),
                "R" => Ok(Operation::Reject),
                _ => Ok(Operation::Jump(s.to_string())),
            }
        }
    }
}

fn main() {
    let input = include_str!("input");

    let one = part_one(input);
    println!("Part one: {}", one);
}

#[test]
fn test () {
    let example = include_str!("example");

    let sum = part_one(example);    
    assert_eq!(19114, sum);
}

fn part_one(input: &str) -> i32 {
    let (datas, functions) = read(input);

    let mut sum = 0;
    for data in datas {
        let entry_point = "in";
        let mut function = functions[entry_point].iter();
        let mut operation = function.next().unwrap();

        let accepted: bool = loop {
            break match operation {
                Operation::LessThan(prop, num, fun) => {
                    let value = data.get(prop);
                    if value < *num {
                        operation = fun.to_owned();
                    } else {
                        operation = function.next().unwrap();
                    }

                    continue;
                }
                Operation::MoreThan(prop, num, fun) => {
                    let value = data.get(prop);
                    if value > *num {
                        operation = fun.to_owned();
                    } else {
                        operation = function.next().unwrap();
                    }

                    continue;
                }
                Operation::Reject => false,
                Operation::Accept => true,
                Operation::Jump(fun) => {
                    function = functions[fun.as_str()].iter();
                    operation = function.next().unwrap();
                    continue;
                }
            };
        };

        if accepted {
            println!("Data accepted: {:?}", data);

            let num = data.a + data.m + data.s + data.x;
            sum += num;
        }
    }

    sum
}

fn read(input: &str) -> (Vec<Data>, HashMap<&str, Vec<Operation>>) {
    let mut datas = Vec::new();
    let mut functions = HashMap::new();

    let mut reading_data = false;
    for line in input.lines() {
        if reading_data {
            let data = line.replace("=", "\":");
            let data = data.replace(",", ",\"");
            let data = data.replace("{", "{\"");

            let data: Data = serde_json::from_str(&data).unwrap();
            println!("Data: {:?}", data);
            datas.push(data);
        } else {
            if line.is_empty() {
                reading_data = true;
                continue;
            }

            let start = line.chars().position(|c| c == '{').unwrap();
            let end = line.chars().position(|c| c == '}').unwrap();

            let name = line.get(0..start).unwrap();
            let operations = line.get(start + 1..end).unwrap();
            let operations = operations
                .split(',')
                .map(Operation::from_str)
                .collect::<Result<Vec<_>, ParseOprError>>()
                .unwrap();

            println!("Instruction: {}, {:?}", name, operations);

            functions.insert(name, operations);
        }
    }

    (datas, functions)
}
