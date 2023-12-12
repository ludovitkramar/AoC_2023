fn main() {
    let input = include_str!("input");

    let sum = part_one(input);
    println!("Part one: {}", sum);

    let sum_two = part_two(input, 1000000);
    println!("Part two: {}", sum_two);
}

fn part_one(input: &str) -> u64 {
    let mut data = read(input);
    expand(&mut data);

    let galaxies = get_galaxies(&data);
    let pairs = get_pairs(&galaxies);
    let mut sum = 0;
    for pair in pairs {
        let distance = calculate_distance(&pair);

        sum += distance;
    }
    sum
}

fn part_two(input: &str, expansion_index: u64) -> u64 {
    let data = read(input);

    let empty_rows = get_empty_rows(&data);
    let empty_cols = get_empty_cols(&data);

    let galaxies = get_galaxies(&data);
    let pairs = get_pairs(&galaxies);
    let mut sum = 0;
    for pair in pairs {
        let distance = calculate_distance(&pair);

        let start_x = std::cmp::min(pair.0.col, pair.1.col);
        let end_x = std::cmp::max(pair.0.col, pair.1.col);

        let start_y = std::cmp::min(pair.0.row, pair.1.row);
        let end_y = std::cmp::max(pair.0.row, pair.1.row);

        let crossed_cols_count = empty_cols
            .iter()
            .filter(|col| start_x < **col && end_x > **col)
            .count() as u64;

        let crossed_rows_count = empty_rows
            .iter()
            .filter(|row| start_y < **row && end_y > **row)
            .count() as u64;

        sum += distance + (crossed_cols_count + crossed_rows_count) * (expansion_index - 1);
    }
    sum
}

fn calculate_distance(pair: &(&Galaxy, &Galaxy)) -> u64 {
    let x = (pair.0.col as i64 - pair.1.col as i64).abs() as u64;
    let y = (pair.0.row as i64 - pair.1.row as i64).abs() as u64;

    return x + y;
}

fn get_pairs<T>(data: &Vec<T>) -> Vec<(&T, &T)> {
    let mut pairs = Vec::new();

    for (i, value) in data.iter().enumerate() {
        for other in data.iter().skip(i + 1) {
            pairs.push((value, other));
        }
    }

    let combinations = combinations(data.len() as u64, 2);
    assert_eq!(combinations, pairs.len() as u64);

    pairs
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Galaxy {
    row: usize,
    col: usize,
}

fn get_galaxies(data: &Vec<Vec<bool>>) -> Vec<Galaxy> {
    let mut galaxies = Vec::new();

    for (i, row) in data.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value {
                galaxies.push(Galaxy { row: i, col: j });
            }
        }
    }

    galaxies
}

fn get_empty_rows(data: &Vec<Vec<bool>>) -> Vec<usize> {
    let mut indices = Vec::new();
    for (i, row) in data.iter().enumerate() {
        if !row.iter().any(|is_galaxy| *is_galaxy) {
            indices.push(i);
        }
    }
    indices
}

fn get_empty_cols(data: &Vec<Vec<bool>>) -> Vec<usize> {
    let row_length = data.iter().next().and_then(|row| Some(row.len())).unwrap();

    let mut indices = Vec::new();
    for i in 0..row_length {
        if !data
            .iter()
            .map(|x| x.get(i).unwrap())
            .any(|is_galaxy| *is_galaxy)
        {
            indices.push(i);
        }
    }
    indices
}

fn expand(data: &mut Vec<Vec<bool>>) {
    let mut indices = Vec::new();
    let mut row_length = 0;

    for (i, row) in data.iter().enumerate() {
        if !row.iter().any(|is_galaxy| *is_galaxy) {
            indices.push(i + indices.len());

            if row_length == 0 {
                row_length = row.len();
            }
        }
    }

    println!("Expanding {} rows.", indices.len());

    for index in indices {
        data.insert(index, vec![false; row_length])
    }

    let mut indices = Vec::new();

    for i in 0..row_length {
        if !data
            .iter()
            .map(|x| x.get(i).unwrap())
            .any(|is_galaxy| *is_galaxy)
        {
            indices.push(i + indices.len());
        }
    }

    println!("Expanding {} columns.", indices.len());

    for index in indices {
        for row in data.iter_mut() {
            row.insert(index, false);
        }
    }
}

fn read(input: &str) -> Vec<Vec<bool>> {
    let mut data = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push(char == '#');
        }
        data.push(row);
    }

    data
}

/// n: Total items, k: Set size
fn combinations(n: u64, k: u64) -> u64 {
    let a: u64 = (n - k + 1..=n).product();
    let b: u64 = (1..=k).product();

    return a / b;
}

#[test]
fn test_combinations() {
    let a = combinations(9, 2);
    assert_eq!(a, 36);

    let a = combinations(100, 2);
    assert_eq!(a, 4950);

    let a = combinations(100, 4);
    assert_eq!(a, 3_921_225);
}

#[test]
fn test() {
    let example = include_str!("example");
    assert_eq!(part_one(example), 374);

    let input = include_str!("input");
    assert_eq!(part_one(input), 9545480);

    assert_eq!(part_two(example, 10), 1030);
    assert_eq!(part_two(example, 100), 8410);
}
