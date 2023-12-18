use std::str::FromStr;

#[derive(Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Data {
    dir: Direction,
    steps: u32,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug)]
struct ParseDirError;
impl FromStr for Direction {
    type Err = ParseDirError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = match s.chars().next().ok_or(ParseDirError)? {
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'U' => Direction::Up,
            _ => panic!(),
        };

        Ok(dir)
    }
}

fn main() {
    let input = include_str!("example");

    let area = part_one(&input);
    println!("Total area: {}", area);

    println!("===========================");

    let area = part_two(input);
    println!("Part two: {}", area);
}

#[test]
fn test() {
    let example_input = include_str!("example");
    let test_input = include_str!("test");
    let input_input = include_str!("input");

    let area = part_one(&example_input);
    assert_eq!(area, 62);

    let area = part_one(&test_input);
    assert_eq!(area, 44);

    let area = part_one(&input_input);
    assert_eq!(area, 28911);
}

fn part_one(input: &str) -> usize {
    let data = read_part_one(input);
    let perimeter = get_perimeter(&data);

    calculate_area(perimeter)
}

fn part_two(input: &str) -> usize {
    let data = read_part_two(input);
    let perimeter = get_perimeter(&data);

    calculate_area(perimeter)
}

fn calculate_area(perimeter: Vec<Position>) -> usize {
    let max_x = perimeter.iter().max_by(|&a, &b| a.x.cmp(&b.x)).unwrap();
    let max_y = perimeter.iter().max_by(|&a, &b| a.y.cmp(&b.y)).unwrap();

    let min_x = perimeter.iter().min_by(|&a, &b| a.x.cmp(&b.x)).unwrap();
    let min_y = perimeter.iter().min_by(|&a, &b| a.y.cmp(&b.y)).unwrap();

    let width = max_x.x - min_x.x + 1;
    let height = max_y.y - min_y.y + 1;

    println!("Size of map: {{w: {}, h: {}}}", width, height);

    let mut map = vec![vec![None::<&Position>; height as usize]; width as usize];

    println!("Min x: {}, Max x: {}", min_x.x, max_x.x);
    println!("Min y: {}, Max y: {}", min_y.y, max_y.y);

    for node in perimeter.iter() {
        let x = node.x - min_x.x;
        let y = node.y - min_y.y;

        map[x as usize][y as usize] = Some(node);
    }

    let mut in_x = 0;
    let mut in_y = 0;
    'out: for y in 0..(height as usize) {
        let mut inside = false;
        let mut in_wall = false;

        for x in 0..(width as usize) {
            let value = map[x][y];

            if value.is_some() {
                if !in_wall {
                    inside = !inside;
                }

                if in_wall {
                    break;
                }

                in_wall = true;
                continue;
            }

            in_wall = false;

            if inside {
                println!("x: {}, y: {}", x, y);
                in_x = x;
                in_y = y;
                break 'out;
            }
        }
    }

    let mut area = 0;
    let mut to_visit = vec![(in_x, in_y)];
    let mut visited = vec![vec![false; height as usize]; width as usize];
    loop {
        if to_visit.is_empty() {
            break;
        }

        let current = to_visit.remove(0);
        let x = current.0;
        let y = current.1;

        if visited[x][y] {
            continue;
        }

        visited[x][y] = true;
        area += 1;

        // take all neighbours that are not visited and are not borders.
        if !(map[x + 1][y].is_some() || visited[x + 1][y]) {
            to_visit.push((x + 1, y));
        }
        if !(map[x - 1][y].is_some() || visited[x - 1][y]) {
            to_visit.push((x - 1, y));
        }
        if !(map[x][y + 1].is_some() || visited[x][y + 1]) {
            to_visit.push((x, y + 1));
        }
        if !(map[x][y - 1].is_some() || visited[x][y - 1]) {
            to_visit.push((x, y - 1));
        }
    }

    let perimeter_length = perimeter.len();
    println!("Inside area = {}", area);
    println!("Perimeter length = {}", perimeter_length);

    area + perimeter_length
}

fn get_perimeter(data: &Vec<Data>) -> Vec<Position> {
    let mut perimeter = Vec::new();

    let mut last_pos = Position { x: 0, y: 0 };

    for entry in data {
        // let color = entry.color.clone();

        for _i in 0..entry.steps {
            let new_pos = match entry.dir {
                Direction::Right => Position {
                    x: last_pos.x + 1,
                    y: last_pos.y.clone(),
                },
                Direction::Down => Position {
                    x: last_pos.x.clone(),
                    y: last_pos.y + 1,
                },
                Direction::Left => Position {
                    x: last_pos.x - 1,
                    y: last_pos.y.clone(),
                },
                Direction::Up => Position {
                    x: last_pos.x.clone(),
                    y: last_pos.y - 1,
                },
            };

            last_pos = new_pos.clone();
            perimeter.push(new_pos);
        }

        println!("{:?}", entry);
    }

    let before = perimeter.len();
    perimeter.dedup_by(|a, b| a.x == b.x && a.y == b.y);
    let after = perimeter.len();
    assert_eq!(before, after);

    perimeter
}

fn read_part_one(input: &str) -> Vec<Data> {
    let mut data = Vec::new();

    for line in input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
    {
        match line[..] {
            [dir, count, color] => {
                let dir = dir.parse::<Direction>().unwrap();
                let count = count.parse::<u32>().unwrap();
                let mut color = color.to_string();
                color.retain(|char| char.is_alphanumeric());

                let color = hex::decode(color).unwrap();

                data.push(Data { dir, steps: count })
            }
            _ => panic!(),
        }
    }

    data
}

fn read_part_two(input: &str) -> Vec<Data> {
    let mut data = Vec::new();

    for line in input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
    {
        match line[..] {
            [_dir, _count, color] => {
                let mut color = color.to_string();
                color.retain(|char| char.is_alphanumeric());

                let dir = color.remove(5);
                // 0 means R, 1 means D, 2 means L, and 3 means U.
                let dir = match dir {
                    '0' => Direction::Right,
                    '1' => Direction::Down,
                    '2' => Direction::Left,
                    '3' => Direction::Up,
                    _ => panic!(),
                };

                let count = u32::from_str_radix(color.as_str(), 16).unwrap();
                let d = Data { dir, steps: count };

                println!("{:?}", d);

                data.push(d)
            }
            _ => panic!(),
        }
    }

    data
}
