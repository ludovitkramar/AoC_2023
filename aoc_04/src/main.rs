use std::collections::HashMap;

fn parse(input: &str) -> Vec<u32> {
    let mut res = Vec::new();

    for part in input.split(" ").map(|p| p.trim()) {
        let n = part.parse::<u32>();
        if n.is_ok() {
            res.push(n.unwrap());
        }
    }

    return res;
}

fn main() {
    let input = include_str!("input");
    let points = part_one(input);

    println!("\nTotal: {}", points);

    let two = part_two(input);
    println!("Part two: {}", two);
}

fn count_matches(line: &str) -> usize {
    let t0: Vec<&str> = line.split("|").collect();
    let card: Vec<&str> = t0.get(0).unwrap().split(":").collect();
    let my_numbers = t0.get(1).unwrap();

    let winning_numbers = card.get(1).unwrap();

    let win = parse(&winning_numbers);
    let mut my = parse(&my_numbers);

    my.retain(|n| win.contains(n));

    my.len()
}

fn part_one(input: &str) -> i32 {
    let mut points = 0;

    for line in input.lines() {
        let count = count_matches(line);
        if count > 0 {
            let point = 2_i32.pow((count - 1).try_into().unwrap());
            points += point;
        }
    }

    points
}

fn part_two(input: &str) -> usize {
    let mut card_count = 0;
    let mut extras = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        if !line.contains("Card") {
            continue;
        }
        card_count += 1;

        let count = count_matches(line);

        let current_id = i + 1;
        let mut instances_of_current = 1;
        match extras.get(&current_id) {
            Some(value) => instances_of_current += value,
            _ => (),
        };

        println!(
            "Card [{}] has {} instances.",
            current_id, instances_of_current
        );

        if count > 0 {
            let new_cards = current_id + 1..=current_id + count;            

            for id in new_cards {
                match extras.get(&id) {
                    Some(value) => extras.insert(id, value + 1 * instances_of_current),
                    None => extras.insert(id, 1 * instances_of_current),
                };

                card_count += 1 * instances_of_current;
            }
        }
    }

    card_count
}

#[test]
fn test() {
    let example = include_str!("example");
    let example_one = part_one(example);
    assert_eq!(example_one, 13);

    let input = include_str!("input");
    let input_one = part_one(input);
    assert_eq!(input_one, 25231);

    let example_two = part_two(example);
    assert_eq!(example_two, 30);

    let input_two = part_two(input);
    assert_eq!(input_two, 9721255);
}
