fn main() {
    let input = include_str!("input");
    let numbers = std::collections::HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut sum = 0;
    let mut parsed_sum: u32 = 0;
    for line in input.lines() {
        {
            let mut num: u32 = 0;

            let mut normalized_line: String = line.to_ascii_lowercase();
            for number_name in numbers.keys() {
                normalized_line = normalized_line
                    .replace(number_name, &numbers.get(number_name).unwrap().to_string());
            }

            for char in normalized_line.chars() {
                if char.is_digit(10) {
                    num = char.to_digit(10).unwrap() * 10;
                    break;
                }
            }
            for char in normalized_line.chars().rev() {
                if char.is_digit(10) {
                    num += char.to_digit(10).unwrap();
                    break;
                }
            }
            sum += num;

            // println!("Normalized result: {}, {}, {}", line, normalized_line, num);
        }

        // Parsing method.
        let mut parsed_num: u32 = 0;
        'forward: for (i, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                parsed_num = char.to_digit(10).unwrap() * 10;
                break 'forward;
            } else {
                // if char is first character of number name key, check if the next characters also match the
                // name of the key.
                // if so, take the number it represents and break.

                let matches = numbers.keys().filter(|n| n.chars().nth(0).unwrap() == char);
                for n in matches {
                    let length = n.len();
                    let s = line.get(i..i + length);

                    if s.is_some() && &s.unwrap() == n {
                        parsed_num = numbers.get(n).unwrap() * 10;
                        // println!("found number!: {}, {}, {}", line, s.unwrap(), parsed_num);
                        break 'forward;
                    }
                }
            }
        }
        'backward: for (i, char) in line.char_indices().rev() {
            // println!("iter: {}, {}", i, char);
            if char.is_digit(10) {
                parsed_num += char.to_digit(10).unwrap();
                // println!("found digit {}", char);
                break 'backward;
            } else {
                let matches = numbers
                    .keys()
                    .filter(|n| n.chars().nth_back(0).unwrap() == char);
                for n in matches {
                    // println!("backward matches: {}, {}", n, char);

                    let length = n.len();
                    if length > i {
                        continue;
                    }

                    let s = line.get(i - length + 1..i + 1);

                    if s.is_some() && &s.unwrap() == n {
                        parsed_num += numbers.get(n).unwrap();
                        // println!("found backwars: {}, {}, {}, {}", line, s.unwrap(), parsed_num, char);
                        break 'backward;
                    } else {
                        // println!("no match: {}, {}", s.unwrap(), n);
                    }
                }
            }
        }
        parsed_sum += parsed_num;

        println!("Parsed result: {}, {}", line, parsed_num);
    }

    println!("Done!");
    println!("Parsed result: {}", parsed_sum);
    println!("Normalized result: {}", sum);
}
