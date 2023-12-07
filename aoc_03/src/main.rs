use std::collections::HashSet;

// (start, end, value), at end index it's something else.
fn get_numbers_from_line(line: &str) -> Vec<(usize, usize, u32)> {
    let mut numbers: Vec<(usize, usize, u32)> = Vec::new();

    let mut start_index = 0;
    let mut matching_number = false;
    for (j, char) in line.chars().enumerate() {
        if char.is_numeric() && !matching_number {
            start_index = j;
            matching_number = true;
        }

        if !char.is_numeric() && matching_number {
            let end_index = j;
            matching_number = false;

            let num = line
                .get(start_index..end_index)
                .unwrap()
                .parse::<u32>()
                .unwrap();

            numbers.push((start_index, end_index, num));
        }
    }
    // if line ends with a number
    if matching_number {
        let num = line.get(start_index..).unwrap().parse::<u32>().unwrap();
        numbers.push((start_index, line.len(), num));
    }

    return numbers;
}

fn get_indices_around_region(start: usize, end: usize, line: usize) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::new();

    if start > 0 {
        positions.push((line, start - 1));
    }
    positions.push((line, end));

    let range = if start > 0 {
        start - 1..=end
    } else {
        start..=end
    };

    if line > 0 {
        for pos in range.clone() {
            positions.push((line - 1, pos))
        }
    }

    for pos in range.clone() {
        positions.push((line + 1, pos));
    }

    return positions;
}

fn main() {
    let input = include_str!("input");
    let sum = do_part_one(input);

    println!("Result: {}", sum);

    let part_2 = part_two(input);
    println!("Part two result: {}", part_2);
}

fn do_part_one(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;

    for (i, line) in lines.iter().enumerate() {
        println!("{}", line);

        let numbers = get_numbers_from_line(line);
        println!("{:?}", numbers);

        for number in numbers {
            let indices = get_indices_around_region(number.0, number.1, i);
            for position in indices {
                let x = lines.get(position.0);
                match x {
                    Some(x) => {
                        let y = x.chars().nth(position.1);
                        match y {
                            Some(y) => {
                                if y != '.' && !y.is_numeric() {
                                    sum += number.2;
                                    break;
                                }
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }
        }
    }

    sum
}

fn part_two(input: &str) -> u32 {
    // Add up all the mulpiples of number around a gear. (Only when there are exactly two numbers around a gear)
    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..

    let mut sum = 0;
    let mut numbers = Vec::new();

    for line in input.lines() {
        let nums = get_numbers_from_line(line);
        numbers.push(nums);
    }

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '*' {
                let to_check = get_indices_around_region(j, j + 1, i);

                let mut matches = HashSet::new();

                for index in to_check {
                    let nums = numbers.get(index.0);
                    if nums.is_some() {
                        let nums = nums.unwrap();
                        for num in nums {
                            let start = num.0;
                            let end = num.1;

                            if index.1 >= start && index.1 < end {
                                matches.insert(num);
                            }
                        }
                    }
                }

                let mut mult = 0;

                if matches.len() == 2 {
                    for val in matches.into_iter() {
                        if mult == 0 {
                            mult = val.2;
                        } else {
                            mult *= val.2;
                        }
                    }
                }

                sum += mult;
            }
        }
    }

    sum
}

#[test]
fn test() {
    let example = include_str!("example");
    let example_sum = do_part_one(example);

    assert_eq!(example_sum, 4361);

    let input = include_str!("input");
    let input_sum = do_part_one(input);
    assert_eq!(input_sum, 525181);

    // part_two example should be: 467835
    let example_2 = part_two(example);
    assert_eq!(example_2, 467835);
}

#[test]
fn test_n_from_line() {
    let a = get_numbers_from_line("....123");
    let b = get_numbers_from_line("...456.");

    assert!(a.len() == 1);
    assert!(b.len() == 1);

    let aa = a.get(0).unwrap();
    let bb = b.get(0).unwrap();

    assert_eq!(bb.2, 456);
    assert_eq!(bb.0, 3);
    assert_eq!(bb.1, 6);

    assert_eq!(aa.2, 123);
    assert_eq!(aa.0, 4);
    assert_eq!(aa.1, 7);
}
