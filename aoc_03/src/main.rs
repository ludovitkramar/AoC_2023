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
        numbers.push((start_index, line.len() - 1, num));
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

    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;

    for (i, line) in lines.iter().enumerate() {
        println!("{}", line);

        let numbers = get_numbers_from_line(line);
        println!("{:?}", numbers);

        for number in numbers {
            let indices = get_indices_around_region(number.0, number.1, i);
            println!("Number: {}, Indices: {:?}", number.2, indices);
            for position in indices {
                let x = lines.get(position.0);
                match x {
                    Some(x) => {
                        let y = x.chars().nth(position.1);
                        match y {
                            Some(y) => {
                                if y != '.' && !y.is_numeric() {
                                    sum += number.2;
                                    println!("Adding: {}", number.2);
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

    // assert_eq!(sum, 4361);
    println!("Result: {}", sum);
}
