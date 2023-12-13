use std::num::ParseIntError;

fn main() {
    let input = include_str!("input");
    let data = read(input);

    let sum = part_one(&data);
    println!("Ans: {}", sum);
}

#[test]
fn test() {
    let example = include_str!("example");
    let example_data = read(example);

    let sum = part_one(&example_data);
    assert_eq!(sum, 21);

    let input = include_str!("input");
    let input_data = read(input);

    let sum = part_one(&input_data);
    assert_eq!(sum, 7195);
}

fn part_one(data: &Vec<Record>) -> u64 {
    let mut sum = 0;
    for entry in data {
        let unknown_count = entry
            .data
            .iter()
            .filter(|x| **x == Condition::Unkown)
            .count();

        let all_possibilities = get_all_possibilities(&entry.data);

        let possibilities_count = 2u64.pow(unknown_count as u32);
        let mut debug_counter = 0;

        let mut counter = 0;
        for possibiliy in all_possibilities {
            assert_eq!(possibiliy.len(), entry.data.len());
            let ok = check(&possibiliy, &entry.check);
            if ok {
                counter += 1;
            }

            debug_counter += 1;
        }
        assert_eq!(debug_counter, possibilities_count);

        println!("There are {} possible answers.", counter);
        sum += counter;
    }
    sum
}

fn expand_possibilities(
    previous: Option<Box<dyn Iterator<Item = Vec<Condition>>>>,
) -> Box<dyn Iterator<Item = Vec<Condition>>> {
    match previous {
        Some(previous) => {
            let temp = previous.flat_map(|x| {
                [
                    [(x).clone(), vec![Condition::Operational]].concat(),
                    [(x).clone(), vec![Condition::Damaged]].concat(),
                ]
            });

            Box::new(temp.into_iter().chain(std::iter::empty()))
        }
        None => Box::new(
            vec![vec![Condition::Operational], vec![Condition::Damaged]]
                .into_iter()
                .chain(std::iter::empty()),
        ),
    }
}

fn get_all_possibilities(data: &Vec<Condition>) -> Box<dyn Iterator<Item = Vec<Condition>> + '_> {
    let unknown_count = data
        .iter()
        .filter(|condition| **condition == Condition::Unkown)
        .count();

    if unknown_count == 0 {
        return Box::new(std::iter::empty());
    }

    let mut start = expand_possibilities(None);

    for _ in 1..unknown_count {
        start = expand_possibilities(Some(start));
    }

    let ret = start.map(|entry| {
        let mut index = 0;
        data.iter()
            .map(|value| match value {
                Condition::Unkown => {
                    let v = (*entry.get(index).unwrap()).clone();
                    index += 1;
                    v
                }
                _ => (*value).clone(),
            })
            .collect::<Vec<_>>()
    });

    Box::new(ret)
}

fn check(data: &Vec<Condition>, checksum: &Vec<u32>) -> bool {
    let mut counts = Vec::new();

    let mut count: u32 = 0;
    for value in data {
        match value {
            Condition::Operational => {
                if count > 0 {
                    counts.push(count);
                }
                count = 0;
            }
            Condition::Damaged => {
                count += 1;
            }
            Condition::Unkown => panic!(),
        }
    }
    if count > 0 {
        counts.push(count);
    }

    if counts.len() != checksum.len() {
        return false;
    }

    !counts.iter().zip(checksum).any(|value| value.0 != value.1)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Condition {
    Operational,
    Damaged,
    Unkown,
}

impl Condition {
    fn parse(symbol: char) -> Self {
        match symbol {
            '#' => Condition::Damaged,
            '.' => Condition::Operational,
            _ => Condition::Unkown,
        }
    }
}

struct Record {
    data: Vec<Condition>,
    check: Vec<u32>,
}

fn read(input: &str) -> Vec<Record> {
    let mut entries = Vec::new();

    for line in input.lines() {
        match line
            .split(' ')
            .into_iter()
            .map(|x| x.trim())
            .collect::<Vec<_>>()[..]
        {
            [data, checksum] => {
                let mut conditions = Vec::new();
                for symbol in data.chars() {
                    conditions.push(Condition::parse(symbol));
                }

                let checksum = checksum
                    .split(',')
                    .into_iter()
                    .map(|x| x.parse::<u32>())
                    .collect::<Result<Vec<_>, ParseIntError>>();

                entries.push(Record {
                    data: conditions,
                    check: checksum.unwrap(),
                });
            }
            _ => {
                panic!()
            }
        }
    }

    entries
}
