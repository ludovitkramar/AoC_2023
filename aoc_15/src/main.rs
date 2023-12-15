use std::collections::HashMap;

fn main() {
    let input = include_str!("input");

    let sum = part_one(input);
    println!("Sum of hashes: {}", sum);

    let sum = part_two(input);
    println!("Part two: {}", sum);
}

fn part_one(input: &str) -> u32 {
    input.split(",").map(|x| x.trim()).map(hash).sum()
}

fn hash(string: &str) -> u32 {
    let mut hash = 0;

    for char in string.chars() {
        let ascii: u32 = char.into();
        hash += ascii;
        hash *= 17;
        hash %= 256;
    }

    assert!(hash <= u8::MAX.into());
    println!("Hashing: {}. Hash: {}", string, hash);

    hash
}

#[derive(Debug)]
enum Operation {
    Remove,
    Insert,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

fn part_two(input: &str) -> u32 {
    let mut hashmap: HashMap<u32, Vec<Lens>> = HashMap::new();

    for value in input.split(",").map(|x| x.trim()) {
        let mut label = String::new();
        let mut data = String::new();
        let mut reading_data = false;
        let mut operation = None;
        for char in value.chars() {
            if reading_data {
                data.push(char);
                continue;
            }

            match char {
                '=' => {
                    reading_data = true;
                    operation = Some(Operation::Insert)
                }
                '-' => {
                    reading_data = true;
                    operation = Some(Operation::Remove)
                }

                _ => label.push(char),
            }
        }

        println!("Label: {}, Data: {}", label, data);

        let box_id = hash(label.as_str());
        let focal_length = data.parse::<u8>();
        
        match operation.unwrap() {
            Operation::Remove => match hashmap.get_mut(&box_id) {
                Some(list) => list.retain(|x| x.label != label),
                None => (),
            },
            Operation::Insert => {
                let lens = Lens {
                    label: label.clone(),
                    focal_length: focal_length.unwrap(),
                };

                match hashmap.get_mut(&box_id) {
                    Some(list) => {
                        let position = list.iter().position(|x| x.label == label);

                        match position {
                            Some(position) => {
                                list.remove(position);
                                list.insert(position, lens);
                            }
                            None => list.push(lens),
                        }
                    }
                    None => {
                        hashmap.insert(box_id, vec![lens]);
                    }
                };
            }
        }

        println!("Hashmap: {:?}", hashmap);
    }

    let mut sum: u32 = 0;

    for key in hashmap.keys() {
        let value = &hashmap[key];

        let box_mult = key + 1;
        for (i, lens) in value.iter().enumerate() {
            sum += box_mult * ((i as u32) + 1) * lens.focal_length as u32;
        }
    }

    sum
}

#[test]
fn test() {
    let example = include_str!("example");
    let sum = part_one(example);

    assert_eq!(sum, 1320);

    let sum = part_two(example);
    assert_eq!(sum, 145);
}
