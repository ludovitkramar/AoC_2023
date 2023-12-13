use std::fmt::{Display, Write};

fn main() {
    let input = include_str!("input");
    let data = read(input);

    let sum = part_one(&data);

    println!("Part one: {}", sum);
}

fn part_one(data: &Vec<Puzzle>) -> usize {
    let mut sum = 0;

    for puzzle in data {
        let equal_cols = puzzle.get_equal_cols();
        let equal_rows = puzzle.get_equal_rows();

        for row in equal_rows {
            if is_perfect_reflection(&puzzle.rows, row) {
                sum += row * 100;
            }
        }

        for col in equal_cols {
            if is_perfect_reflection(&puzzle.cols, col) {
                sum += col;
            }
        }
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
    let example_data = read(example);

    let sum = part_one(&example_data);
    assert_eq!(sum, 405);

    let input = include_str!("input");
    let input_data = read(input);

    let sum = part_one(&input_data);
    assert_eq!(sum, 39939);
}
