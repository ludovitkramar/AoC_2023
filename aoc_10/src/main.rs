use std::collections::HashSet;

fn main() {
    let input = include_str!("input");

    let a = part_one(input);
    println!("Result: {}", a);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Position {
    fn can_move(&self, dir: Direction) -> bool {
        match dir {
            Direction::Down => true,
            Direction::Up => self.y > 0,
            Direction::Left => self.x > 0,
            Direction::Right => true,
        }
    }

    fn move_to(&self, dir: &Direction) -> Self {
        let mut new = self.clone();

        match dir {
            Direction::Down => new.y += 1,
            Direction::Up => new.y -= 1,
            Direction::Left => new.x -= 1,
            Direction::Right => new.x += 1,
        }

        new
    }

    fn around(&self) -> Vec<Direction> {
        let mut ret = Vec::new();

        ret.push(Direction::Down);
        ret.push(Direction::Right);

        if self.can_move(Direction::Up) {
            ret.push(Direction::Up);
        }

        if self.can_move(Direction::Left) {
            ret.push(Direction::Left);
        }

        ret
    }

    fn is_connected(&self, dir: &Direction, symbol: &char) -> bool {
        let symbol = *symbol;
        match dir {
            Direction::Up => symbol == '|' || symbol == '7' || symbol == 'F',
            Direction::Down => symbol == '|' || symbol == 'L' || symbol == 'J',
            Direction::Left => symbol == '-' || symbol == 'F' || symbol == 'L',
            Direction::Right => symbol == '-' || symbol == 'J' || symbol == '7',
        }
    }

    fn query(&self, data: &Vec<&str>) -> Option<char> {
        data.get(self.y).and_then(|line| line.chars().nth(self.x))
    }
}

fn part_one(input: &str) -> i64 {
    let lines = input.lines().collect::<Vec<_>>();

    let start_pos_symbol = 'S';
    let mut start_pos = None;
    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == start_pos_symbol {
                start_pos = Some(Position { x: j, y: i });
                break;
            }
        }
    }

    let start_pos = start_pos.unwrap();
    let mut visited = HashSet::new();
    visited.insert(start_pos);

    let mut next_positions = advance_step(&lines, &start_pos, &mut visited);
    let mut steps = 0;

    while next_positions.len() > 0 {
        steps += 1;
        let mut new_next = Vec::new();
        for pos in next_positions {
            let mut new_next_part = advance_step(&lines, &pos, &mut visited);
            for temp_debug_only in &new_next_part {
                assert!(!new_next.contains(temp_debug_only));
            }
            new_next.append(&mut new_next_part);
        }
        next_positions = new_next;
    }

    steps
}

fn advance_step(
    data: &Vec<&str>,
    position: &Position,
    visited: &mut HashSet<Position>,
) -> Vec<Position> {
    let mut all_visited = true;
    let mut to_visit = Vec::new();

    assert!(visited.contains(position));

    for dir in position.around() {
        let next_position = position.move_to(&dir);
        let next_symbol = next_position.query(data);

        if next_symbol.is_some() && next_position.is_connected(&dir, &next_symbol.unwrap()) {
            let this_visited = visited.contains(&next_position);
            all_visited = all_visited && this_visited;

            if !this_visited {
                to_visit.push(next_position);
                visited.insert(next_position);
            }
        }
    }

    if all_visited {
        assert!(to_visit.len() == 0);
    } else {
        assert!(to_visit.len() > 0);
    }

    return to_visit;
}

#[test]
fn test() {
    // Sample output

    // ..45.
    // .236.
    // 01.78
    // 14567
    // 23...

    let example = include_str!("example");
    let a = part_one(example);
    assert_eq!(a, 8);

    let input = include_str!("input");
    let a = part_one(input);
    assert_eq!(a, 6754);
}
