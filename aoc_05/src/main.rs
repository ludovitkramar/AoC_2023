use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct MapEntry {
    pub source_start: i64,
    pub range_length: i64,
    pub offset: i64,
}

#[derive(Debug)]
struct ParseMapEntryError;

impl FromStr for MapEntry {
    type Err = ParseMapEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.split(' ').collect();
        let dst_start = input.get(0).ok_or(ParseMapEntryError)?;
        let src_start = input.get(1).ok_or(ParseMapEntryError)?;
        let length = input.get(2).ok_or(ParseMapEntryError)?;

        let destination_start = dst_start.parse::<i64>().map_err(|_| ParseMapEntryError)?;
        let source_start = src_start.parse::<i64>().map_err(|_| ParseMapEntryError)?;
        let range_length = length.parse::<i64>().map_err(|_| ParseMapEntryError)?;

        Ok(MapEntry {
            offset: destination_start - source_start,
            source_start,
            range_length,
        })
    }
}

#[derive(Debug)]
struct Map {
    pub from: String,
    pub to: String,
    pub entries: Vec<MapEntry>,
}

impl Map {
    pub fn convert(&self, num: i64) -> i64 {
        let mut ret = num;
        for entry in &self.entries {
            if num >= entry.source_start && num < entry.source_start + entry.range_length {
                ret = num + entry.offset;
                break;
            }
        }

        ret
    }
}

#[derive(Debug)]
struct LookUpError;

fn look_up(
    from: &str,
    to: &str,
    data: &HashMap<String, Map>,
    input: i64,
) -> Result<i64, LookUpError> {
    let mut map = data.get(&from.to_string()).ok_or(LookUpError)?;
    let mut lookup = map.convert(input);

    loop {
        if map.to == to {
            break;
        }

        map = data.get(&map.to).ok_or(LookUpError)?;
        lookup = map.convert(lookup);
    }

    Ok(lookup)
}

fn look_up_range(
    from: &str,
    to: &str,
    data: &HashMap<String, Map>,
    start: i64,
    count: i64,
) -> Result<i64, LookUpError> {
    
    // Brute force of shame.
    let end = start + count;

    let mut min = i64::MAX;
    for index in start..end {
        let ans = look_up(from, to, data, index).unwrap();
        min = std::cmp::min(min, ans);
    }

    Ok(min)

    // Failed attemp:

    // let mut maps = Vec::new();

    // {
    //     let mut map = data.get(&from.to_string()).ok_or(LookUpError)?;
    //     loop {
    //         maps.push(map);
    //         if map.to == to {
    //             break;
    //         }

    //         map = data.get(&map.to).ok_or(LookUpError)?;
    //     }
    // }

    // let mut min = i64::MAX;

    // let mut indices = Vec::new();
    // let mut first_index = start;
    // let mut last_index = start + count - 1;
    // let mut previous_map: Option<String> = None;

    // for map in &maps {
    //     let current_map = map.from.clone();

    //     if previous_map.is_some() {
    //         let previous = previous_map.unwrap();

    //         let transformed_indices: Vec<i64> = indices.iter().map(|v| {
    //             look_up(&previous, &current_map, data, *v).unwrap()
    //         }).collect();

    //         first_index = transformed_indices.iter().min().unwrap().to_owned();
    //         last_index = transformed_indices.iter().max().unwrap().to_owned();
    //     }

    //     indices = Vec::new();

    //     indices.push(first_index);
    //     indices.push(last_index);
    //     for entry in (&map.entries).iter().filter(|e| {
    //         let section_start = e.source_start;
    //         section_start > first_index && section_start < last_index
    //     }) {
    //         indices.push(entry.source_start);
    //     }

    //     for entry in (&map.entries).iter().filter(|e| {
    //         let section_end = e.source_start + e.range_length - 1;
    //         section_end > first_index && section_end < last_index
    //     }) {
    //         indices.push(entry.source_start + entry.range_length);
    //     }

    //     for index in &indices {
    //         let value = look_up(&current_map, to, data, *index).unwrap();

    //         if value < min {
    //             println!(
    //                 "Map: {}. Index: {}, Value: {}. Min: {min} {}",
    //                 current_map,
    //                 index,
    //                 value,
    //                 value < min
    //             );
    //             min = value;
    //         }

    //         // min = std::cmp::min(min, value);
    //     }

    //     previous_map = Some(current_map);
    // }

    // println!("part two done: {}", min);

    // Ok(min)
}

struct Data {
    pub maps: HashMap<String, Map>,
    pub seeds: Vec<i64>,
}

fn read(input: &str) -> Data {
    let mut maps = HashMap::new();

    let mut seeds = Vec::new();

    let mut reading_map = false;

    let mut map_from: String = String::new();
    let mut map_to: String = String::new();
    let mut map_entries = Vec::new();

    for line in input.lines() {
        if line.starts_with("seeds:") {
            for num in line.split(' ') {
                let n = num.parse::<i64>();
                if n.is_ok() {
                    seeds.push(n.unwrap());
                }
            }

            println!("Seeds: {:?}", seeds);
        }

        if reading_map {
            let entry = line.parse::<MapEntry>();
            if entry.is_ok() {
                map_entries.push(entry.unwrap());
            } else {
                reading_map = false;

                let map = Map {
                    to: map_to.clone(),
                    from: map_from.clone(),
                    entries: map_entries,
                };
                println!("Created map: {:?}", map);

                maps.insert(map_from.clone(), map);
                map_entries = Vec::new();
            }
        }

        if line.ends_with(" map:") {
            assert_eq!(map_entries.len(), 0);

            reading_map = true;

            let name = line[..line.len() - 5].to_string();
            let s: Vec<&str> = name.split('-').collect();
            map_from = s.get(0).unwrap().to_string();
            map_to = s.get(2).unwrap().to_string();

            println!("{}, from: {}, to: {}", name, map_from, map_to);
        }
    }

    let map = Map {
        from: map_from.clone(),
        to: map_to.clone(),
        entries: map_entries,
    };
    println!("Created map: {:?}", map);

    maps.insert(map_from.clone(), map);

    Data { maps, seeds }
}

fn part_one(data: &Data) -> i64 {
    let maps = &data.maps;
    let seeds = &data.seeds;

    let mut min = i64::MAX;
    for seed in seeds {
        let ans = look_up("seed", "location", maps, seed.to_owned());

        println!("Ans: {:?}", ans);

        if ans.is_ok() {
            min = std::cmp::min(min, ans.unwrap());
        }
    }

    min
}

fn part_two(data: &Data) -> i64 {
    let mut min = i64::MAX;

    let maps = &data.maps;
    let seeds = &data.seeds;

    for range in seeds.chunks(2) {
        let start = range.get(0).unwrap().to_owned();
        let count = range.get(1).unwrap().to_owned();

        let ans = look_up_range("seed", "location", maps, start, count).unwrap();
        min = std::cmp::min(min, ans);
    }

    min
}

fn main() {
    let input = include_str!("input");
    let data = read(input);

    let part1 = part_one(&data);
    println!("Part 1: Minimim location: {}.", part1);

    let part2 = part_two(&data);
    println!("Part 2: Minimim location: {}.", part2);
}

#[test]
fn test_part_one() {
    let data = read(include_str!("input"));
    let min = part_one(&data);

    assert_eq!(107430936, min);
}

#[test]
fn test_example() {
    let data = read(include_str!("example"));
    let maps = &data.maps;

    assert_eq!(look_up("seed", "soil", maps, 79).unwrap(), 81);
    assert_eq!(look_up("seed", "soil", maps, 14).unwrap(), 14);
    assert_eq!(look_up("seed", "soil", maps, 55).unwrap(), 57);
    assert_eq!(look_up("seed", "soil", maps, 13).unwrap(), 13);

    // Seed number 79 corresponds to soil number 81.
    // Seed number 14 corresponds to soil number 14.
    // Seed number 55 corresponds to soil number 57.
    // Seed number 13 corresponds to soil number 13.

    assert_eq!(look_up("seed", "fertilizer", maps, 79).unwrap(), 81);
    assert_eq!(look_up("seed", "fertilizer", maps, 14).unwrap(), 53);
    assert_eq!(look_up("seed", "fertilizer", maps, 55).unwrap(), 57);
    assert_eq!(look_up("seed", "fertilizer", maps, 13).unwrap(), 52);

    assert_eq!(look_up("seed", "location", maps, 79).unwrap(), 82);
    assert_eq!(look_up("seed", "location", maps, 14).unwrap(), 43);
    assert_eq!(look_up("seed", "location", maps, 55).unwrap(), 86);
    assert_eq!(look_up("seed", "location", maps, 13).unwrap(), 35);

    // Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
    // Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
    // Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
    // Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.

    let part1_min = part_one(&data);
    let part2_min = part_two(&data);

    assert_eq!(part1_min, 35);
    assert_eq!(part2_min, 46);
}
