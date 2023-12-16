use std::fmt::{Display, Write};

fn main() {
    let input = include_str!("input");
    let mut data = read(input);

    let sum = part_one(&data);
    println!("Part one: {}", sum);

    let sum = part_two(&mut data);
    println!("Part two: {}", sum);
}

fn part_one(data: &Vec<Puzzle>) -> usize {
    let mut sum = 0;

    for puzzle in data {
        sum += puzzle.calculate_reflection_line(None);
    }

    sum
}

fn part_two(data: &mut Vec<Puzzle>) -> usize {
    let mut sum = 0;

    for puzzle in data.iter_mut() {
        let original = puzzle.calculate_reflection_line(None);        

        let rows = &puzzle.rows;
        let cols = &puzzle.cols;
        let col_count = cols.len();
        let total_count = col_count * rows.len();

        let mut new_reflection = 0;
        for i in 0..total_count {
            let x = i % col_count;
            let y = i / col_count;

            let mut modified_rows = puzzle.rows.clone();
            let mut modified_cols = puzzle.cols.clone();

            let row = modified_rows.get_mut(y).unwrap();
            let col = modified_cols.get_mut(x).unwrap();

            let original_char = row.chars().nth(x).unwrap();
            let replace_with = match original_char {
                '#' => ".",
                '.' => "#",
                _ => panic!(),
            };

            row.replace_range(x..x + 1, replace_with);
            col.replace_range(y..y + 1, replace_with);

            let modified_puzzle = Puzzle {
                rows: modified_rows.clone(),
                cols: modified_cols.clone(),
            };
            modified_puzzle.verify();

            let reflection = modified_puzzle.calculate_reflection_line(Some(original));
            if reflection != original && reflection != 0 {
                println!("New reflection: {}.", reflection);

                if original < 100 {
                    if reflection < 100 {
                        // both less than 100
                        new_reflection = reflection;
                    } else {
                        // new is more than one hundred
                        new_reflection = reflection - reflection % 100
                    }
                } else {
                    if reflection < 100 {
                        new_reflection = reflection;
                    } else {
                        // both more or equal to 100
                        let big = reflection - reflection % 100;
                        let small = reflection % 100;
                        if big != original {
                            new_reflection = big;
                        } else if small != 0 {
                            new_reflection = small;
                        } else {
                            panic!();
                        }
                    }
                }

                assert!(new_reflection != 0);
                break;
            }
        }

        if new_reflection == 0 {
            println!("Couldn't find new reflection!");
            panic!();
        }
        sum += new_reflection;
    }

    sum
}

struct Puzzle {
    rows: Vec<String>,
    cols: Vec<String>,
}

impl Puzzle {
    fn get_equal_cols(&self) -> Vec<usize> {
        get_adjacent_equal_indices(&self.cols)
    }

    fn get_equal_rows(&self) -> Vec<usize> {
        get_adjacent_equal_indices(&self.rows)
    }

    fn verify(&self) {
        let row_count = self.rows.len();
        let col_count = self.cols.len();

        for col in self.cols.iter() {
            assert_eq!(col.len(), row_count);
        }

        for row in self.rows.iter() {
            assert_eq!(row.len(), col_count);
        }

        for (i, row) in self.rows.iter().enumerate() {
            for (j, col) in self.cols.iter().enumerate() {
                let row_char = row.chars().nth(j).unwrap();
                let col_char = col.chars().nth(i).unwrap();

                assert_eq!(row_char, col_char);
            }
        }
    }

    fn calculate_reflection_line(&self, ignore: Option<usize>) -> usize {
        let equal_cols = self.get_equal_cols();
        let equal_rows = self.get_equal_rows();
        let mut sum = 0;

        for col in equal_cols {
            if is_perfect_reflection(&self.cols, col) {
                let value = col;
                match ignore {
                    Some(ignore) => {
                        if ignore != value {
                            sum += value;
                            break;
                        }
                    }
                    None => {
                        sum += value;
                        break;
                    }
                }
            }
        }

        for row in equal_rows {
            if is_perfect_reflection(&self.rows, row) {
                let value = row * 100;
                match ignore {
                    Some(ignore) => {
                        if ignore != value {
                            sum += value;
                            break;
                        }
                    }
                    None => {
                        sum += value;
                        break;
                    }
                }
            }
        }

        sum
    }
}

fn get_adjacent_equal_indices<T: PartialEq>(input: &Vec<T>) -> Vec<usize> {
    let mut previous = None;
    let mut indices = Vec::new();

    for (i, row) in input.iter().enumerate() {
        match previous {
            Some(previous) => {
                if previous == row {
                    indices.push(i);
                }
            }
            None => (),
        };

        previous = Some(row);
    }

    indices
}

fn is_perfect_reflection<T: PartialEq>(input: &Vec<T>, index: usize) -> bool {
    let mut pairs = Vec::new();

    for i in 0..index {
        let left = index - i - 1;
        let right = index + i;

        if right > input.len() - 1 {
            break;
        }

        pairs.push((left, right));
    }

    let imperfect = pairs.iter().any(|pair| {
        let left = input.get(pair.0).unwrap();
        let right = input.get(pair.1).unwrap();
        left != right
    });

    !imperfect
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            let _ = f.write_char('\n');
            for char in row.chars() {
                let _ = f.write_char(char);
            }
        }

        std::fmt::Result::Ok(())
    }
}

fn parse(puzzle: &Vec<&str>) -> Puzzle {
    let rows: Vec<String> = puzzle.iter().map(|row| row.to_string()).collect();
    let mut cols = Vec::new();

    let col_count = rows.first().unwrap().len();
    for i in 0..col_count {
        let mut col = String::new();
        for row in &rows {
            let char = row.chars().nth(i).unwrap();
            col.push(char);
        }
        cols.push(col);
    }

    Puzzle { rows, cols }
}

fn read(input: &str) -> Vec<Puzzle> {
    let mut ret = Vec::new();
    let mut puzzle = Vec::new();

    for line in input.lines() {
        if line.len() == 0 {
            ret.push(parse(&puzzle));
            puzzle = Vec::new();
        } else {
            puzzle.push(line);
        }
    }

    if puzzle.len() > 0 {
        ret.push(parse(&puzzle));
    }

    ret
}

#[test]
fn test() {
    let example = include_str!("example");
    let mut example_data = read(example);

    let sum = part_one(&example_data);
    assert_eq!(sum, 405);

    let input = include_str!("input");
    let mut input_data = read(input);

    let sum = part_one(&input_data);
    assert_eq!(sum, 39939);

    let sum = part_two(&mut example_data);
    assert_eq!(sum, 400);

    let sum = part_two(&mut input_data);
    assert_eq!(sum, 32069);
}
