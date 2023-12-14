fn main() {
    let input = include_str!("input");

    let sum = part_one(input);
    println!("Part one: {sum}");

    let sum = part_two(input);
    println!("Part two: {sum}");
}

#[derive(Debug)]
struct MoveUpError;

fn move_all_up(lines: &mut Vec<Vec<char>>) {
    let mut moved_count = i32::MAX;
    while moved_count > 0 {
        moved_count = 0;
        for i in (0..lines.len()).rev() {
            let line = lines.get(i).unwrap().clone();
            let has_previous = i > 0;
            if has_previous {
                for (j, char) in line.iter().enumerate() {
                    if *char == 'O' {
                        let moved = try_move_up(i, j, lines).unwrap();
                        if moved {
                            moved_count += 1;
                        }
                    }
                }
            }
        }
    }
}

fn try_move_up(row: usize, col: usize, lines: &mut Vec<Vec<char>>) -> Result<bool, MoveUpError> {
    let previous = lines.get_mut(row - 1).ok_or(MoveUpError)?;
    let previous_char = previous.get(col).ok_or(MoveUpError)?;

    let should_move = *previous_char == '.';
    if should_move {
        previous[col] = 'O';

        let current = lines.get_mut(row).ok_or(MoveUpError)?;
        current[col] = '.';
    }

    Ok(should_move)
}

fn move_all_down(lines: &mut Vec<Vec<char>>) {
    let mut moved_count = i32::MAX;
    while moved_count > 0 {
        moved_count = 0;
        for i in 0..lines.len() - 1 {
            let line = lines.get(i).unwrap().clone();

            for (j, char) in line.iter().enumerate() {
                if *char == 'O' {
                    let moved = try_move_down(i, j, lines).unwrap();
                    if moved {
                        moved_count += 1;
                    }
                }
            }
        }
    }
}

fn try_move_down(row: usize, col: usize, lines: &mut Vec<Vec<char>>) -> Result<bool, MoveUpError> {
    let next = lines.get_mut(row + 1).ok_or(MoveUpError)?;
    let next_char = next.get(col).ok_or(MoveUpError)?;

    let should_move = *next_char == '.';
    if should_move {
        next[col] = 'O';

        let current = lines.get_mut(row).ok_or(MoveUpError)?;
        current[col] = '.';
    }

    Ok(should_move)
}

fn move_all_left(lines: &mut Vec<Vec<char>>) {
    let col_count = lines.first().and_then(|x| Some(x.len())).unwrap();

    let mut moved_count = i32::MAX;
    while moved_count > 0 {
        moved_count = 0;
        for i in 0..col_count - 1 {
            for row in lines.iter_mut() {
                let char = row.get_mut(i).unwrap();
                if *char == '.' {
                    let next = row.get_mut(i + 1).unwrap();
                    if *next == 'O' {
                        row[i + 1] = '.';
                        row[i] = 'O';
                        moved_count += 1;
                    }
                }
            }
        }
    }
}

fn move_all_right(lines: &mut Vec<Vec<char>>) {
    let col_count = lines.first().and_then(|x| Some(x.len())).unwrap();

    let mut moved_count = i32::MAX;
    while moved_count > 0 {
        moved_count = 0;
        for i in 0 + 1..col_count {
            for row in lines.iter_mut() {
                let char = row.get_mut(i).unwrap();
                if *char == '.' {
                    let previous = row.get_mut(i - 1).unwrap();
                    if *previous == 'O' {
                        row[i - 1] = '.';
                        row[i] = 'O';
                        moved_count += 1;
                    }
                }
            }
        }
    }
}

fn cycle(lines: &mut Vec<Vec<char>>) {
    move_all_up(lines);
    move_all_left(lines);
    move_all_down(lines);
    move_all_right(lines);
}

fn count_load(lines: &Vec<Vec<char>>) -> usize {
    let line_count = lines.len();
    let mut sum = 0;
    for (i, line) in lines.iter().enumerate() {
        let count = line.iter().filter(|x| **x == 'O').count();

        // println!(
        //     "Line {} has {} rocks. [{}]",
        //     i,
        //     count,
        //     line.into_iter().collect::<String>()
        // );

        sum += count * (line_count - i);
    }

    sum
}

fn part_one(input: &str) -> usize {
    let mut lines: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    move_all_up(&mut lines);

    count_load(&lines)
}

fn part_two(input: &str) -> usize {
    let mut lines: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    for i in 0..1000000000 {
        cycle(&mut lines);

        let load = count_load(&lines);
        println!("[{}] Load: {}", i, load);
    }

    count_load(&lines)
}

#[test]
fn test_part_one() {
    let example = include_str!("example");
    let input = include_str!("input");

    let sum = part_one(example);
    assert_eq!(sum, 136);

    let sum = part_one(input);
    assert_eq!(sum, 107430);
}

#[test]
fn test_part_two() {
    let example = include_str!("example");

    let sum = part_two(example);
    assert_eq!(sum, 64);
}
